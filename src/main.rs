use std::env;
mod new_lexical;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argumento = args[1].clone();

    let cadena: String = String::from(argumento);
    let mut analizer: new_lexical::LexicalAnalysis = new_lexical::LexicalAnalysis::new(&cadena);
    analizer.analize();
    analizer.get_results();
}
