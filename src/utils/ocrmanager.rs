use rust_fuzzy_search::fuzzy_compare;

use threadpool::ThreadPool;
use std::sync::{mpsc::channel, Arc, Mutex, Condvar};

use super::epub_utils;

#[derive(Debug)]
struct Page {
    //page: String, useless, since it's not accessed
    similarity: f32,
    chapter_number: usize,
    chapter_page_number: usize
}


//function that, given a pic of a physical book page, gives the corresponding page in the ebook
pub fn get_ebook_page(ebook_name: String, physical_page: String) -> Option<usize> {

    //OCR PHASE: Load the LEPTESS model, set the image to the leptess model, get the text
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(physical_page).unwrap();
    let text = lt.get_utf8_text().unwrap();

    //EBOOK PHASE: Setup book path and get chapter numbers through the metadata
    let book_path = format!("saved_books/{}", ebook_name);
    let book_metadata = epub_utils::get_metadata_of_book(book_path.as_str());
    let chapters_number = book_metadata["chapters"].parse::<usize>().unwrap();

    //THREAD PHASE: Create a thread pool and a channel
    let pool = ThreadPool::new(4);
    let (tx, rx) = channel();

    //Setup condition variable holding a usize: this will allow us to save the page number of the best match
    let pair: Arc<(Mutex<Option<usize>>, Condvar)> = Arc::new((Mutex::new(Some(0)), Condvar::new()));

    //This will contain the pages of each chapter, to calculate the global page number
    let chapters_pages_numbers = Arc::new(Mutex::new(vec![0; chapters_number]));

    //For each chapter..
    for i in 0..chapters_number {
        let tx = tx.clone();

        let book_path_clone = book_path.clone();
        let text_clone = text.clone();
        let chapters_pages_numbers_clone = chapters_pages_numbers.clone();

        //..create a thread that will calculate the similarity between the physical page and the chapter pages
        //NOTE: the thread pool will aggregate these functions in 4 threads (see pool initialization)
        pool.execute(move || {
            let result = compute_similarity(book_path_clone, text_clone, i, chapters_pages_numbers_clone);
            tx.send(result).expect("Error in unwrapping");
        });
    }

    //Clone the pair, to pass it to the thread
    let pair_clone = pair.clone();

    //This thread receives all the results of the previous threads, finding the best page
    //After this, it will wake up the main thread (see below)
    std::thread::spawn(move || {
        //Get all the results from the channel, and filter the ones that are not None
        let results: Vec<Option<Page>> = rx.iter().take(chapters_number).collect();
        let results_some = results.iter().filter(|x| x.is_some()).collect::<Vec<_>>();

        let mut to_return = None;

        //If there are some.. (pun)
        if results_some.len() > 0 {
            //.. get the one with the highest similarity
            let best_match_page = results_some.iter().max_by(|&a, &b| {
                let a = a.as_ref().unwrap();
                let b = b.as_ref().unwrap();
                a.similarity.partial_cmp(&b.similarity).unwrap()
            });
        
            //Calculate the global page number and return it
            let mut pages_sum = 0;
            for i in 0..best_match_page.unwrap().as_ref().unwrap().chapter_number {
                pages_sum += chapters_pages_numbers.lock().unwrap()[i];
            }
            pages_sum += best_match_page.unwrap().as_ref().unwrap().chapter_page_number;

            //set the data inside the condition variable to pages_sum
            to_return = Some(pages_sum);
        }

        let (lock, cvar) = &*pair_clone;
        let mut data = lock.lock().unwrap();
        *data = to_return;
        cvar.notify_one();
    });

    //Go to sleep until the thread sends a notification
    let (lock, cvar) = &*pair;
    let mut page_number = lock.lock().unwrap();
    while *page_number == Some(0) {
        page_number = cvar.wait(page_number).unwrap();
    }

    return *page_number;
}


fn compute_similarity(book_path: String, text: String, chapter_to_examine: usize, chapter_pages_number: Arc<Mutex<Vec<usize>>>) -> Option<Page> {

    let chapter_pages = epub_utils::split_chapter_in_vec(book_path.as_str(), None, chapter_to_examine, 8, 0);

    //add the number of pages of the chapter to the vector chapter_pages_number
    let mut chapter_pages_number = chapter_pages_number.lock().unwrap();
    chapter_pages_number[chapter_to_examine] = chapter_pages.len();

    //Iterate through pages_vec
    for i in 0..chapter_pages.len() {

        let page = &chapter_pages[i];

        //Take the first 10 words of text and page and join them into a lowercase string
        let text_words: Vec<&str> = text.split_whitespace().take(10).collect();
        let text_substring = text_words.join(" ").to_lowercase();
        let page_words: Vec<&str> = page.split_whitespace().take(10).collect();
        let page_substring = page_words.join(" ").to_lowercase();

        //Compute the similarity between the two strings
        let mut similarity = fuzzy_compare(text_substring.as_str(), page_substring.as_str());

        //If the similarity is less than 0.8, skip the page
        if similarity < 0.8 {
            continue;
        }

        //While the similarity remains higher than 0.8, keep comparing the next 10 words and so on
        let mut i = 20;
        while similarity > 0.8 {
            let text_words: Vec<&str> = text.split_whitespace().skip(i-10).take(i).collect();
            let text_substring = text_words.join(" ").to_lowercase();
            let page_words: Vec<&str> = page.split_whitespace().skip(i-10).take(i).collect();
            let page_substring = page_words.join(" ").to_lowercase();
            similarity = fuzzy_compare(text_substring.as_str(), page_substring.as_str());

            i += 10;
            //If we reached the end of the page or the text, return the page
            if i > text_words.len() || i > page_words.len() {

                return Some(Page {
                    similarity,
                    chapter_number: chapter_to_examine,
                    chapter_page_number: i
                });
            }
        }

        //If we reach this point, that means the similarity dropped below 0.8 at a certain point
        //Thus we skip this page and go to the next one
    }

    //No page had a similarity higher than 0.8, return None
    return None;
}


pub fn get_physical_page() {

    /*
    Secondo me quello che vuole nel secondo punto è:
    1. Fai due foto al libro cartaceo (potrebbero essere la prima pagina del capitolo 1 e l’ultima pagina dell’ultimo capitolo 
    2. Ottieni informazioni delle pagine tramite OCR
    2.1 ottieni numero di caratteri per pagina (permette di capire quanto testo ci va in una pagina fisica, così risolvi il problema dell’impaginazione ecc)
    2.2 ottieni il numero della pagina (così sai quante sono le pagine in totale in cui c’è testo del libro)
    3. Stima la posizione del testo che hai nella tua pagina dell’ebook nel libro fisico

    TRADOTTO
    1. Il metodo prende due path in input, prima e ultima pagina cartacea
    2. Prendi il numero di caratteri per pagina e il numero di pagine totali
    3. Prendi il testo dall'ebook e STIMA una pagina cartacea
    
    Per fare questa stima bisogna considerare:
    - il numero di caratteri per pagina del cartaceo
    - Il numero di caratteri nell'ebook fino alla pagina virtuale in questione
    - Ad esempio se abbiamo 30 caratteri per ogni pagina cartacea, e nell'ebook siamo arrivati a 100 caratteri, allora la pagina cartacea in cui ci siamo è 100/30 = 3.33

    Calcoli più difficili ma più precisi(forse) possono includere il numero di caratteri per ogni pagina cartacea e il numero di caratteri PER OGNI PAGINA VIRTUALE
    */

    todo!();
}