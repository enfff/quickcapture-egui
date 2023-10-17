# QuickCapture
A multi platform screenshot utility written in Rust

## Choses à faire
- [ ] Caricare mostrare solo l'immagine appena scattata, magari salvandola con un nome fisso
- [ ] Per ora bisogna rendere il codice modulare. attualmente se premo il tasto "take a screenshot" lui esegue una funzione scritta brutalmente dentro il bottone. invece vorrei che richiami una funzione, magari scritta in un altro modulo/file, che gestisce quella funzionalità
- [ ] Trovare un modo per nascondere la finestra prima dello screenshot, fare lo screenshot, e infine rimostrare tutte le finestre
- [x] Caricare un widget per mostrare lo screenshot appena scattato 

## Docs
[Egui Docs](https://docs.rs/egui/latest/egui/)
[Tutorial](https://youtu.be/NtUkr_z7l84)

## Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`