/*
* Reto #1
* ¿ES UN ANAGRAMA?
* Fecha publicación enunciado: 03/01/22
* Fecha publicación resolución: 10/01/22
* Dificultad: MEDIA
*
* Enunciado: Escribe una función que reciba dos palabras (String) y retorne verdadero o falso (Boolean) según sean o no anagramas.
* Un Anagrama consiste en formar una palabra reordenando TODAS las letras de otra palabra inicial.
* NO hace falta comprobar que ambas palabras existan.
* Dos palabras exactamente iguales no son anagrama.
*
* Información adicional:
* - Usa el canal de nuestro discord (https://mouredev.com/discord) "🔁reto-semanal" para preguntas, dudas o prestar ayuda a la acomunidad.
* - Puedes hacer un Fork del repo y una Pull Request al repo original para que veamos tu solución aportada.
* - Revisaré el ejercicio en directo desde Twitch el lunes siguiente al de su publicación.
* - Subiré una posible solución al ejercicio el lunes siguiente al de su publicación.
*
*/

#[allow(dead_code)]
fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1 == word2 {
        return false;
    }
    let mut chars1: Vec<char> = word1.to_lowercase().chars().collect();
    let mut chars2: Vec<char> = word2.to_lowercase().chars().collect();
    chars1.sort();
    chars2.sort();

    chars1 == chars2
}

fn main() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_same_words() {
        assert_eq!(is_anagram("hola", "hola"), false)
    }

    #[test]
    fn test_valid_anagram() {
        assert_eq!(is_anagram("saco", "caso"), true);
        assert_eq!(is_anagram("amor", "Roma"), true);
    }

    #[test]
    fn test_invalid_anagram() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(is_anagram("saco", "casco"), false);
    }
}
