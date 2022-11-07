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

    //the text variable contains a book page: there can be words splitted between lines, so join them
    //also remove all new lines, making the text a single big string
    let text = lt.get_utf8_text().unwrap().replace("-\n", "").replace("\n", " ");

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

    let chapter_pages = epub_utils::split_chapter_in_vec(book_path.as_str(), None, chapter_to_examine, 8, 0, 800.0, 300.0);

    //add the number of pages of the chapter to the vector chapter_pages_number
    let mut chapter_pages_number = chapter_pages_number.lock().unwrap();
    chapter_pages_number[chapter_to_examine] = chapter_pages.len();

    //Iterate through che chapter pages
    for i in 0..chapter_pages.len() {

        //replace all \n characters with spaces. the \n characters may be attached to words
        let page = &chapter_pages[i].replace("\n", " ");

        //if the page is empty, skip it
        if page.len() == 0 {
            continue;
        }

        let mut j = 0;
        let mut similarity;

        //calculate the TOTAL number of words in the text and in the page
        let text_words_number = text.split_whitespace().count();
        let page_words_number = page.split_whitespace().count();

        //Continue while the similarity is higher than 0.7 and the we aren't left with less than 5 words in the text or in the page
        loop {

            println!("----------------");

            let text_words: Vec<&str> = text.split_whitespace().clone().skip(j).take(10).collect();
            let text_substring = text_words.join(" ");
            let page_words: Vec<&str> = page.split_whitespace().clone().skip(j).take(10).collect();
            let page_substring = page_words.join(" ");

            similarity = fuzzy_compare(text_substring.as_str(), page_substring.as_str());

            //print text substring and page substring
            println!("text: {}", text_substring);
            println!("page: {}", page_substring);
            println!("j: {}, text_words_number: {}, page_words_number: {}", j, text_words_number, page_words_number);
            println!("similarity: {}", similarity);
           
            j += 10;
            if !(similarity > 0.6 && text_words_number-5 > j && page_words_number-5 > j) {
                break;
            }
        }

        if similarity > 0.6 {
            return Some(Page {
                chapter_number: chapter_to_examine,
                chapter_page_number: i,
                similarity: similarity,
            });
        }
    }

    //No page had a similarity higher than 0.8, return None
    return None;
}


//This method needs to be completed and tested
pub fn get_physical_page(first_physical_page_path: String, second_physical_page_path: String, actual_ebook_page: String, actual_ebook_page_number: usize) -> usize {

    todo!();

    //OCR PHASE: Load the LEPTESS model, get the two physical pages texts
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(first_physical_page_path).unwrap();
    let first_text = lt.get_utf8_text().unwrap();
    lt.set_image(second_physical_page_path).unwrap();
    let second_text = lt.get_utf8_text().unwrap();

    //get the number of characters of the first PHYSICAL page
    let first_text_chars = first_text.chars().count();

    //get the number of characters of the current EBOOK page
    let actual_ebook_page_chars = actual_ebook_page.chars().count();

    //Multiply the number of chars in the actual ebook page by the page number of the actual ebook page
    //We'll get the total amount of characters in the ebook until the actual page
    //Divide this quantity by the number of chars contained in a single physical page
    //We'll get the page number of the physical page we're looking for
    let physical_page_number = (actual_ebook_page_chars * actual_ebook_page_number) / first_text_chars;

    return physical_page_number;

}