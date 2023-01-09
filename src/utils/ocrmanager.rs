use rust_fuzzy_search::fuzzy_compare;

use std::sync::{mpsc::channel, Arc, Mutex, Condvar};

use crate::models::book::{PAGE_WIDTH, PAGE_HEIGHT};

use super::epub_utils;

#[derive(Debug)]
struct Page {
    chapter_number: usize,
    chapter_page_number: usize
}


//function that, given a pic of a physical book page, gives the corresponding page in the ebook
pub fn get_ebook_page(ebook_name: String, physical_page: String, font_size: f64) -> Option<(usize,usize)> {

    //start timer
    let start = std::time::Instant::now();

    //OCR PHASE: Load the LEPTESS model, set the image to the leptess model, get the text
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(physical_page).unwrap();

    //the "text" variable contains a book page: there can be words splitted between lines, so join them
    //also remove all new lines, making the text a single big string
    let text = lt.get_utf8_text().unwrap().replace("-\n", "").replace("\n", " ");

    //EBOOK PHASE: Setup book path and get chapter numbers through the metadata
    let book_path = format!("saved_books/{}", ebook_name);
    let book_metadata = epub_utils::get_metadata_of_book(book_path.as_str());
    let chapters_number = book_metadata["chapters"].parse::<usize>().unwrap();

    //THREAD PHASE: Create a thread pool and a channel
    let pool = threadpool::Builder::new().build();
    let (tx, rx) = channel();

    //Setup condition variable holding a usize: this will allow us to save the page number of the best match
    let pair: Arc<(Mutex<Option<(usize,usize)>>, Condvar)> = Arc::new((Mutex::new(Some((0,0))), Condvar::new()));

    //For each chapter..
    for i in 0..chapters_number {
        let tx = tx.clone();

        let book_path_clone = book_path.clone();
        let text_clone = text.clone();

        //..create a thread that will calculate the similarity between the physical page and the chapter pages
        //NOTE: the thread pool will aggregate these functions in 4 threads (see pool initialization)
        pool.execute(move || {
            let result = compute_similarity(book_path_clone, text_clone, i, font_size);
            if result.is_some() {
                tx.send(result.unwrap()).expect("Error in sending msg");
            }
        });
    }

    //Clone the pair, to pass it to the thread
    let pair_clone = pair.clone();

    //This thread receives the found page and calculates the global page number
    //After this, it will wake up the main thread (see below)
    std::thread::spawn(move || {

        let mut to_return = None;

        //create a duration: 10 seconds
        let duration = std::time::Duration::from_secs(10);

        //If a found page is found in "duration" seconds..
        if let Ok(found_page) = rx.recv_timeout(duration) {
            //Save the chapter and page number
            to_return = Some((found_page.chapter_number, found_page.chapter_page_number));
        }

        //Notify the main thread, wheter we had a match or not
        let (lock, cvar) = &*pair_clone;
        let mut data = lock.lock().unwrap();
        *data = to_return;
        cvar.notify_one();
    });

    //Go to sleep until the receiver thread sends a notification
    let (lock, cvar) = &*pair;
    let mut page_number = lock.lock().unwrap();
    while *page_number == Some((0,0)) {
        page_number = cvar.wait(page_number).unwrap();
    }

    //Stop timer
    let duration = start.elapsed();
    println!("Time elapsed in get_ebook_page() is: {:?}", duration);

    return *page_number;
}


//This function, given a chapter, gets its pages and iterates through them.
//For each page, it computes the similarity with the given text: if it's higher than 0.85, the page is returned
fn compute_similarity(book_path: String, text: String, chapter_to_examine: usize, font_size: f64) -> Option<Page> {

    let chapter_pages = epub_utils::split_chapter_in_vec(book_path.as_str(), None, chapter_to_examine, 8, font_size, PAGE_WIDTH, PAGE_HEIGHT);

    //Iterate through che chapter pages
    for i in 0..chapter_pages.len() {


        //replace all \n characters with spaces. the \n characters may be attached to words
        let page = &chapter_pages[i].replace("\n", " ");

        //if the page is empty, skip it
        if page.len() == 0 {
            continue;
        }

        let similarity;
        //check which one is longer: PAGE OR TEXT
        if page.len() > text.len() {
            similarity = fuzzy_compare(&text, page);
        }
        else {
            similarity = fuzzy_compare(page, &text);
        }

        //println!("similarity: {}", similarity);

        if similarity > 0.85 {
            return Some(Page {
                chapter_number: chapter_to_examine,
                chapter_page_number: i
            });
        }
    }

    //No page had a similarity higher than 0.85, return None
    return None;
}


pub fn get_physical_page(physical_page_path: String, chapter_number: usize, ebook_char_count: usize) -> usize {

    //OCR PHASE: Load the LEPTESS model, get the two physical pages texts
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(physical_page_path).unwrap();
    let physical_page_text = lt.get_utf8_text().unwrap();

    //get the number of characters of the PHYSICAL page
    let physical_page_chars = physical_page_text.chars().count();

    //Divide the number of chars till now in the ebook by the number of chars contained in a single physical page
    //--> We'll get the page number of the physical page we're looking for
    let mut physical_page_number = ebook_char_count / physical_page_chars;

    //For each chapter, there is a page which has - typically - very few chars with respect to the first physical page (last page of chapter)
    //--> So we need to add 1 from the physical page number each chapter
    //We also need to consider that the first page of each chapter typically has a header
    //--> So we need to add 1 to the physical page number each chapter
    physical_page_number = physical_page_number + (chapter_number*2);

    //print all variables
    println!("------------------------------------");
    println!("first_text_chars: {}", physical_page_chars);
    println!("cumulative: {} ", ebook_char_count);
    println!("physical_page_number: {}", physical_page_number);
    println!("------------------------------------");

    return physical_page_number;

}


/* TEST METHODS */
#[cfg(test)]
mod tests {

    use super::*;
    use serial_test::serial;

    #[test]
    //This method is used to test the fuzzy_compare() method
    fn test_fuzzy_compare() {

        //create an array of fake pages (string)
        //create a string of text to search for
        //find the index of the array with the highets similarity
        //assert that the index is the same as the one we expect
        let pages = vec![
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa",
            "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores "
        ];

        let text = "natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam,";

        let mut max_similarity = 0.0;
        let mut max_similarity_index = 0;

        for i in 0..pages.len() {
            let similarity = fuzzy_compare(&text, &pages[i]);
            if similarity > max_similarity {
                max_similarity = similarity;
                max_similarity_index = i;
            }
        }

        assert_eq!(max_similarity_index, 1);

    }

    //This method is used to test get_ebook_page (small font)
    #[test]
    #[serial]
    fn test_get_ebook_page_small_font() {

        //CASE 1: First page of chapter
        //Search for the page whose ebook version is the first page (index 0) of the sixth chapter (index 5)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok1.png".to_string(), 14.0);
        assert_eq!(page, Some((5,0)));

        //CASE 2: Random page of chapter
        //Search for the page whose ebook version is the 21st page (index 20) of the eleventh chapter (index 10)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok2.png".to_string(), 14.0);
        assert_eq!(page, Some((10,20)));

        //CASE 3: Last page of chapter
        //Search for the page whose ebook version is the last page (index 59) of the eight chapter (index 7)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok3.png".to_string(), 14.0);
        assert_eq!(page, Some((7,59)));


        //CASE 4: Page non-existent in ebook
        //Search for a page that is not in the ebook version
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/err_screenshot.png".to_string(), 14.0);
        assert_eq!(page, None);

    }

    //This method is used to test get_ebook_page (medium font)
    #[test]
    #[serial]
    fn test_get_ebook_page_medium_font() {

        //CASE 1: First page of chapter
        //Search for the page whose ebook version is the first page (index 0) of the sixth chapter (index 5)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok1.png".to_string(), 18.0);
        assert_eq!(page, Some((5,0)));

        //CASE 2: Random page of chapter
        //Search for the page whose ebook version is the 36th page (index 35) of the eleventh chapter (index 10)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok2.png".to_string(), 18.0);
        assert_eq!(page, Some((10,35)));

        //CASE 3: Last page of chapter
        //Search for the page whose ebook version is the last page (index 100) of the eight chapter (index 7)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok3.png".to_string(), 18.0);
        assert_eq!(page, Some((7,100)));


        //CASE 4: Page non-existent in ebook
        //Search for a page that is not in the ebook version
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/err_screenshot.png".to_string(), 18.0);
        assert_eq!(page, None);

    }

    //This method is used to test get_ebook_page (large font)
    #[test]
    #[serial]
    fn test_get_ebook_page_large_font() {

        //CASE 1: First page of chapter
        //Search for the page whose ebook version is the first page (index 0) of the sixth chapter (index 5)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok1.png".to_string(), 22.0);
        assert_eq!(page, Some((5,0)));

        //CASE 2: Random page of chapter
        //Search for the page whose ebook version is the 59st page (index 58) of the eleventh chapter (index 10)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok2.png".to_string(), 22.0);
        assert_eq!(page, Some((10,58)));

        //CASE 3: Last page of chapter
        //Search for the page whose ebook version is the last page (index 156) of the eight chapter (index 7)
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/svevo_ok3.png".to_string(), 22.0);
        assert_eq!(page, Some((7,156)));


        //CASE 4: Page non-existent in ebook
        //Search for a page that is not in the ebook version
        let page = get_ebook_page("svevo_la_coscienza_di_zeno".to_string(), "./test_ocr_images/OCR/err_screenshot.png".to_string(), 22.0);
        assert_eq!(page, None);

    }

    #[test]
    fn test_get_physical_page() {
        
            //CASE 1: First page of chapter
            //Calculate the physical page starting from the first page (chars read: 3654) of the fifth chapter (index 4)
            let page = get_physical_page("./test_ocr_images/OCR_INVERSE/svevo.png".to_string(), 4, 3654);

            //assert in range: the page should be between 9-15 and 9+15 (9 is the real physical page)
            assert!(page <= 9+15);
        
            //CASE 2: Random page of chapter
            //Calculate the physical page starting from the 19th page (chars read: 159737) of the eight chapter (index 7)
            let page = get_physical_page("./test_ocr_images/OCR_INVERSE/svevo.png".to_string(), 7, 159737);

            //assert in range: the page should be between 108-15 and 108+15 (9 is the real physical page)
            assert!(page >= 108-15);
            assert!(page <= 108+15);
        
            //CASE 3: Last page of chapter
            //Search for the page whose ebook version is the last page (chars read: 801152) of the tenth chapter (index 9)
            let page = get_physical_page("./test_ocr_images/OCR_INVERSE/svevo.png".to_string(), 9, 801152);

            //assert in range: the page should be between 142-15 and 142+15 (9 is the real physical page)
            assert!(page >= 545-15);
            assert!(page <= 545+15);

            //NOTE: Case 4 is not present because this method does not calculate the ACTUAL page number: it rather
            //calculates an approximation of it using some heuristics.

            
    }
}