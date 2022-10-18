// Sto mudulo serve solo a prendermi i libri staticamente nel frattemo che sam lo sbrugno finisce di fare le sue cose
//type Book = MockupBook;

// #[derive(Clone, Data, PartialEq)]
// pub struct MockupBook {
// npages: u16,
// read_pages: u16,
// idx: u16,
// selected: bool,
// title: Rc<String>,
// author: Rc<String>,
// cover_path: Rc<String>,
// description: Rc<String>,
// }

// impl GUIBook for MockupBook {
// fn new() -> Self {
// Self {
// npages: 0,
// read_pages: 0,
// idx: 0,
// selected: false,
// title: Rc::new("".to_string()),
// author: Rc::new("".to_string()),
// cover_path: Rc::new("".to_string()),
// description: Rc::new("".to_string()),
// }
// }

// fn get_title(&self) -> Rc<String> {
// self.title.clone()
// }

// fn with_title(mut self, title: impl Into<String>) -> Self {
// self.set_title(title);
// self
// }

// fn set_title(&mut self, title: impl Into<String>) {
// self.title = Rc::new(title.into());
// }

// fn get_author(&self) -> Rc<String> {
// self.author.clone()
// }

// fn with_author(mut self, author: impl Into<String>) -> Self {
// self.set_author(author);
// self
// }

// fn set_author(&mut self, author: impl Into<String>) {
// self.author = Rc::new(author.into());
// }

// fn get_number_of_pages(&self) -> u16 {
// self.npages
// }

// fn with_number_of_pages(mut self, npages: u16) -> Self {
// self.set_number_of_pages(npages);
// self
// }

// fn set_number_of_pages(&mut self, npages: u16) {
// self.npages = npages;
// }

// fn get_number_of_read_pages(&self) -> u16 {
// self.read_pages
// }

// fn with_number_of_read_pages(mut self, read_pages: u16) -> Self {
// self.set_number_of_read_pages(read_pages);
// self
// }

// fn set_number_of_read_pages(&mut self, read_pages: u16) {
// self.read_pages = read_pages;
// }

// fn get_index(&self) -> u16 {
// self.idx
// }

// fn with_index(mut self, idx: u16) -> Self {
// self.set_index(idx);
// self
// }

// fn set_index(&mut self, idx: u16) {
// self.idx = idx
// }

// fn get_cover_path(&self) -> Rc<String> {
// self.cover_path.clone()
// }

// fn with_cover_path(mut self, cover_path: impl Into<String>) -> Self {
// self.set_cover_path(cover_path);
// self
// }

// fn set_cover_path(&mut self, cover_path: impl Into<String>) {
// // TODO: Set to "" if path not found
// let mut path = "".into();
// if let Ok(cwd) = std::env::current_dir() {
// let file_path = cwd.join("src").join("covers").join(cover_path.into());
// if let Some(file_path) = file_path.to_str() {
// path = String::from(file_path);
// }
// }
// self.cover_path = Rc::new(path);
// }

// fn get_description(&self) -> Rc<String> {
// self.description.clone()
// }

// fn with_description(mut self, description: impl Into<String>) -> Self {
// self.set_description(description);
// self
// }

// fn set_description(&mut self, description: impl Into<String>) {
// self.description = Rc::new(description.into());
// }

// fn is_selected(&self) -> bool {
// self.selected == true
// }

// fn set_selected(&mut self, selected: bool) {
// self.selected = selected;
// }

// fn select(&mut self) {
// self.set_selected(true);
// }

// fn unselect(&mut self) {
// self.set_selected(false);
// }
// }

// use std::rc::Rc;

// use druid::{im::Vector, Data, Lens};

// use super::{book::GUIBook, library::GUILibrary};

// fn lotr() -> MockupBook {
// MockupBook::new()
// .with_title("Il Signore degli Anelli")
// .with_number_of_pages(1000)
// .with_number_of_read_pages(247)
// .with_cover_path("lotr.jpg")
// .with_author("J.R.R. Tolkien")
// .with_description("Il Signore degli Anelli è romanzo d'eccezione, al di fuori del tempo: chiarissimo ed enigmatico, semplice e sublime. Dona alla felicità del lettore ciò che la narrativa del nostro secolo sembrava incapace di offrire: avventure in luoghi remoti e terribili, episodi d'inesauribile allegria, segreti paurosi che si svelano a poco a poco, draghi crudeli e alberi che camminano, città d'argento e di diamante poco lontane da necropoli tenebrose in cui dimorano esseri che spaventano solo al nominarli, urti giganteschi di eserciti luminosi e oscuri; e tutto questo in un mondo immaginario ma ricostruito con cura meticolosa, e in effetti assolutamente verosimile, perché dietro i suoi simboli si nasconde una realtà che dura oltre e malgrado la storia: la lotta, senza tregua, fra il bene e il male. Leggenda e fiaba, tragedia e poema cavalleresco, il romanzo di Tolkien è in realtà un'allegoria della condizione umana che ripropone in chiave moderna i miti antichi.")
// }

// fn _1984() -> MockupBook {
// MockupBook::new()
// .with_title("1984")
// .with_number_of_pages(300)
// .with_number_of_read_pages(37)
// .with_cover_path("1984.jpg")
// .with_author("George Orwell")
// .with_description("1984. Il mondo è diviso in tre immensi superstati in perenne guerra fra loro: Oceania, Eurasia ed Estasia. In Oceania la società è governata dall'infallibile e onnisciente Grande Fratello, che nessuno ha mai visto di persona. I suoi occhi sono le telecamere che spiano di continuo nelle case, il suo braccio la Psicopolizia che interviene al minimo sospetto. Non c'è legge scritta e niente, apparentemente, è proibito. Tranne divertirsi. Tranne pensare. Tranne amare. Tranne vivere, insomma. Dal loro rifugio, in uno scenario desolante da Medioevo postnucleare, solo Winston Smith e Julia lottano disperatamente per conservare un granello di umanità…")
// }

// fn sotto_lo_stesso_cielo() -> MockupBook {
// MockupBook::new()
// .with_title("Sotto lo stesso cielo")
// .with_number_of_pages(252)
// .with_number_of_read_pages(252)
// .with_cover_path("sotto-lo-stesso-cielo.jpg")
// .with_author("Giulia Pompili")
// .with_description("«Una mattina ci siamo svegliati e il Secolo asiatico era diventato il Secolo cinese.» La nuova rivalità tra America e Cina, l'aggressiva politica di Xi Jinping spesso ci portano a vedere l'Asia come un'estensione di Pechino, che fagocita e ruba la scena alle nazioni che la circondano. Eppure il ritorno del Dragone sulla scena mondiale, in buona parte, dipende proprio da loro, dalle economie più sviluppate cui la Cina è legata in modo indissolubile. Non esiste Pechino senza Taipei, non esiste Pechino senza Seul, ma soprattutto non esiste Pechino senza Tokyo. La vicinanza geografica, culturale e storica rende praticamente impossibile interpretare i fatti asiatici di oggi – e di conseguenza quelli americani, europei, mondiali – senza conoscere che cosa succede al di là dei confini del Celeste impero. In Asia orientale, infatti, c'è «un clima di eccitante evoluzione, una trasformazione contagiosa». Taiwan inizia a farsi conoscere come un'isola di diritti, di sviluppo, di istruzione, ed è il posto dove trovare la tecnologia più avanzata. In Corea del Sud si può osservare da vicino il sogno di una democrazia che si trasforma in una superpotenza tech e che, con la sua musica e i suoi film, va alla conquista dei grandi festival internazionali, dagli Oscar ai Grammy Awards. Il Giappone, nonostante abbia alle spalle vent'anni di stagnazione economica, ancora oggi in Asia rappresenta l'eccellenza, la raffinatezza, il modello di sviluppo culturale ideale. Attingendo non solo alla storia e ai fatti più recenti, ma anche agli incontri, alle esperienze, ai dialoghi avuti durante i suoi numerosi reportage, la giornalista del «Foglio» Giulia Pompili dedica un libro all'«altra» Asia, ovvero a quella che Cina non è. E da questa prospettiva, globale e personale insieme, mescolando politica, costume e aneddoti, ci racconta il disgelo tra le due Coree, il rilancio del Giappone di Shinzo Abe, il crescente soft power di Seul, l'isola ribelle di Taiwan, ma anche la divisione generazionale e le discriminazioni presenti in queste società ancora fortemente patriarcali. Con la consapevolezza che «l'Asia orientale è un posto complicato, dove il passato torna costantemente nelle cronache contemporanee, dove anche mangiare un gamberetto può essere un atto politico. Ma è l'unico luogo da cui partire per capire lo scontro globale tra America e Cina, e magari trovare una terza via tra due modelli distantissimi».")
// }

// fn california() -> MockupBook {
// MockupBook::new()
// .with_title("California: La Fine del Sogno")
// .with_number_of_pages(204)
// .with_number_of_read_pages(0)
// .with_cover_path("california-la-fine-del-sogno.jpg")
// .with_author("Francesco Costa")
// .with_description("Quando noi italiani pensiamo alla nazione che vorremmo diventare, cosa ci viene in mente?  Probabilmente vorremmo avere un'economia in grande crescita e la piena occupazione: un paese in cui chiunque voglia lavorare possa farlo. Vorremmo avere le migliori università del pianeta e bellezze naturali adeguatamente valorizzate, prodotti culturali dall'influenza globale e la possibilità di definire «made in Italy» non solo un paio di scarpe ma anche un'app capace di costruire il futuro e un'idea che sappia cambiare il mondo. Vorremmo essere il posto ideale per chiunque voglia realizzare i propri sogni, per chiunque abbia un progetto e cerchi le condizioni ideali per trasformarlo in realtà, e magari anche avere una classe dirigente progressista, sensibile, accogliente. Insomma, vorremmo essere un po' più come la California, che infatti da secoli è considerata la «fine del mondo»: un paradiso di tolleranza, prosperità e paesaggi spettacolari, la terra promessa, la più pura incarnazione del sogno americano.  Eppure, in California qualcosa si è inceppato, tanto che da anni le persone che la lasciano sono più di quelle che vi arrivano, e dall'ultimo censimento la sua popolazione risulta per la prima volta diminuita.  Niente di tutto questo dovrebbe accadere, in teoria. Salvo in caso di guerre e catastrofi naturali, nella nostra epoca i movimenti migratori seguono direzioni segnate dall'economia e dall'occupazione: le persone vanno via dai posti che offrono meno opportunità per raggiungere posti che ne offrono di più.  Quella della California è una crisi unica al mondo, ma l'acuta analisi di Francesco Costa ci mostra che le sue ragioni non sono esclusivamente californiane: cominciamo a riscontrarle anche dalle nostre parti.  Le città come unici possibili centri propulsivi della crescita economica. La qualità della vita distrutta dai prezzi delle case. Un radicalismo politico infantile. La divaricazione del mercato del lavoro fra chi possiede un'istruzione di alto livello e chi no. Le discriminazioni razziali. La catastrofe climatica. L'attivismo performativo. Le crescenti diseguaglianze fra generazioni. La crisi della California ci costringe a interrogarci sulla realtà che ci circonda e ci invita a stare attenti a ciò che desideriamo, perché potremmo ottenerlo.")
// }

// fn farenheit() -> MockupBook {
// MockupBook::new()
// .with_title("Farhrenheit 451")
// .with_number_of_pages(177)
// .with_number_of_read_pages(52)
// .with_cover_path("451.jpg")
// .with_author("Ray Bradbury")
// .with_description("Montag fa il pompiere in un mondo in cui ai pompieri non è richiesto di spegnere gli incendi, ma di accenderli: armati di lanciafiamme, fanno irruzione nelle case dei sovversivi che conservano libri e li bruciano. Così vuole fa legge. Montag però non è felice della sua esistenza alienata, fra giganteschi schermi televisivi, una moglie che gli è indifferente e un lavoro di routine. Finché, dall'incontro con una ragazza sconosciuta, inizia per lui la scoperta di un sentimento e di una vita diversa, un mondo di luce non ancora offuscato dalle tenebre della imperante società tecnologica.")
// }

// fn saggio_erotico() -> MockupBook {
// MockupBook::new()
// .with_title("Saggio Erotico sulla Fine del Mondo")
// .with_number_of_pages(276)
// .with_number_of_read_pages(240).with_cover_path("saggio-erotico-sulla-fine-del-mondo.jpg")
// .with_author("Barbascura X")
// .with_description("A un certo punto della loro storia gli esseri umani hanno iniziato a percepire di aver tragicamente incasinato la situazione climatica del proprio pianeta. «Ma come mai nessuno ci ha avvisati prima?» chiesero spaesati in coro, mentre gli scienziati che nell'ultimo secolo avevano cercato di dare l'allarme si accingevano a prendere la rincorsa per tirare ceffoni sul collo all'urlo di «kivemmurt'». Poi arrivò una ragazzina svedese di 15 anni, tale Greta Thunberg, che organizzò uno sciopero e divenne icona mondiale della lotta ai cambiamenti climatici. «Ma allora siete str...!» urlarono gli scienziati. Qualche scettico tra la popolazione si chiese: «Ma perché fanno parlare lei e non parlano mai gli scienziati? Ci dev'essere qualcosa sotto». «Ma allora siete proprio str...» riurlarono gli scienziati, per poi accasciarsi in posizione fetale e morire annegati nelle proprie lacrime. Come è evidente, gli esseri umani sono degli erotomani dell'autodistruzione. Del resto sono i rappresentanti di una specie megalomane che vanta l'invenzione della bretella e delle palline antistress con la faccia di Nicolas Cage, ma soprattutto che è riuscita in pochi secoli a mettere in atto una crisi climatica (quasi) irreversibile. Barbascura X, in questa commedia tragicomica, racconta con sarcasmo e irriverenza il disastro ambientale del nostro tempo attraverso gli occhi di Rino Bretella, l'ultimo malaugurato superstite umano, catapultato casualmente in un futuro molto lontano per un'anomalia spazio-temporale avvenuta nella sua cucina. Un libro che è la parodia di una specie, ma anche il suo manifesto autodistruttivo.")
// }

// fn senza_cover() -> MockupBook {
// MockupBook::new()
// .with_title("Il Miglior Libro Mai Scritto")
// .with_number_of_pages(420)
// .with_number_of_read_pages(69)
// .with_cover_path("IP address di Gab: 115.8.3.1")
// .with_author("Matteo \"Cocco\" Quarta")
// .with_description("Ebbene sì, alla fine ho scritto un libro anche io. Un libro talmente epico da non necessitare di cover. In questo libro racconto come ci sente ad essere me, a sevgliarmi la mattina alle 9 e iniziare a rustare fino alle 9 di sera, a star cercando da quattro mesi casa a Torino senza successo, al non ricevere risposta quando chiedo la tesi, ad aver avviato il trend delle OGR Tech n Tonic e adesso voi schifosi bastardi ci andate e mi mandate pure le foto. È proprio vero che Dio dà le sue battaglie più difficili ai suoi guerrieri più forti, perché io sono fortissimo, nonostante non mi alleni. Sia chiaro, io vorrei allenarmi, ma non è che posso avere 4 abbonamenti attivi a seconda di quale zona d'Italia sarò in quel mese, ziopera.")
// }

// pub fn get_mockup_book_vec() -> Vec<MockupBook> {
// let lotr = lotr();
// let _1984 = _1984();
// let sotto_lo_stesso_cielo = sotto_lo_stesso_cielo();
// let california = california();
// let _451 = farenheit();
// let saggio_erotico = saggio_erotico();
// let senza_cover = senza_cover();

// vec![
// lotr,
// _1984,
// sotto_lo_stesso_cielo,
// california,
// _451,
// saggio_erotico,
// senza_cover,
// ]
// }

use druid::{im::Vector, Data, Lens};

use super::{
    book::{Book, GUIBook},
    library::GUILibrary,
};

#[derive(Clone, Lens, PartialEq, Data)]
pub struct MockupLibrary<B: GUIBook + PartialEq + Data> {
    books: Vector<B>,
    selected_book: Option<u16>,
}

impl GUILibrary<Book> for MockupLibrary<Book> {
    fn new() -> Self {
        Self {
            books: Vector::new(),
            selected_book: None,
        }
    }

    fn add_book(&mut self, book: &Book) {
        self.books
            .push_back(book.clone().with_index(self.books.len()));
    }

    fn remove_book(&mut self, idx: u16) {
        let idx = idx as usize;
        if let Some(_) = self.books.get(idx) {
            self.books.remove(idx);
        }
    }

    fn get_book_mut(&mut self, idx: u16) -> Option<&mut Book> {
        let idx = idx as usize;
        self.books.get_mut(idx)
    }

    fn get_book(&self, idx: u16) -> Option<&Book> {
        let idx = idx as usize;
        self.books.get(idx)
    }

    fn get_selected_book_idx(&self) -> Option<u16> {
        self.selected_book.clone()
    }

    fn number_of_books(&self) -> usize {
        self.books.len()
    }

    fn get_selected_book_mut(&mut self) -> Option<&mut Book> {
        if let Some(idx) = self.get_selected_book_idx() {
            self.get_book_mut(idx)
        } else {
            None
        }
    }

    fn set_selected_book_idx(&mut self, idx: u16) {
        if idx < self.number_of_books() as u16 {
            self.unselect_current_book();
            self.selected_book = Some(idx);
        }
    }

    fn get_selected_book(&self) -> Option<&Book> {
        if let Some(idx) = self.get_selected_book_idx() {
            self.get_book(idx)
        } else {
            None
        }
    }

    fn unselect_current_book(&mut self) {
        if let Some(selected) = self.get_selected_book_mut() {
            selected.unselect();
        }
        self.selected_book = None;
    }
}
