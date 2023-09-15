use regex::Regex;

const QUESTION_PATTERN: &'static str = r#"::::\[choice\](.*?)(\:)?\s*\["#;
const ANSWER_PATTERN: &'static str = r#"\s*([=~])(.*)#"#;

pub struct Chunk(pub Vec<String>);

struct ChunkIterator {
    input: String,
    current_position: u8,
}

impl Iterator for ChunkIterator {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self
            .input
            .lines()
            .take_while(|l| l.len() > 0)
            .map(|l| l.to_string())
            .collect();

        Some(Chunk(chunk))
    }
}

pub struct Question {
    category: String,
    text: String,
    answers: Vec<String>,
    correct_answer: String,
}

pub fn convert() {}

pub fn parse_chunk(
    chunk: Chunk,
    question_matcher: Regex,
    answer_matcher: Regex,
) -> anyhow::Result<Question> {
    let category = chunk.0[1].replace("$CATEGORY:", "");

    let question = {
        let question_line = chunk.0[2].to_string();
        let captures = question_matcher.captures(&question_line).unwrap();
        captures.get(1).unwrap().as_str().to_string()
    };

    let mut correct_answer = 0;
    let answers = {
        let mut answers = vec![];
        chunk
            .0
            .iter()
            .skip(3)
            .enumerate()
            .filter(|(_, line)| line.len() > 1)
            .for_each(|(i, line)| {
                let captures = answer_matcher.captures(line).unwrap();
                if captures.get(1).unwrap().as_str() == "=" {
                    correct_answer = i;
                }
                answers.push(captures.get(1).unwrap().as_str().to_string());
            });
        answers
    };

    Ok(Question {
        category,
        text: question,
        answers,
        correct_answer: char::from_u32(correct_answer as u32 + 65)
            .unwrap()
            .to_string(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chunk_iterator() {
        let input = gift_input_example();
        let mut chunk_iterator = ChunkIterator {
            input: input.to_string(),
            current_position: 0,
        };

        assert_eq!(chunk_iterator.next().unwrap().0.len(), 7);
    }

    #[test]
    fn parse_chunk_into_question() {
        let input = gift_input_example();
        let mut chunk_iterator = ChunkIterator {
            input: input.to_string(),
            current_position: 0,
        };

        let chunk = chunk_iterator.next().unwrap();

        let question_matcher = Regex::new(QUESTION_PATTERN).unwrap();
        let answer_matcher = Regex::new(ANSWER_PATTERN).unwrap();

        let question = parse_chunk(chunk, question_matcher, answer_matcher).unwrap();
        assert_eq!(question.category, "FP_B0_135_esigenze dei consumatori");
        assert_eq!(question.text, "Il premio pagato in buona fede all'intermediario o ad un suo collaboratore si considera come pagato direttamente all'impresa di assicurazione.");
        assert_eq!(question.answers.len(), 3);
        assert_eq!(question.correct_answer, "C");
    }

    fn gift_input_example() -> &'static str {
        r#"// question: 208050  
$CATEGORY:FP_B0_135_esigenze dei consumatori
::::[choice]Il premio pagato in buona fede all'intermediario o ad un suo collaboratore si considera come pagato direttamente all'impresa di assicurazione. [B0_135_04]{
	~Falso# 
	~Vero, ma solo provato con il pagamento presso i locali dell'intermediario# 
	=Vero# 
}

// question: 208078  
$CATEGORY:00A8_035_Intermediazione
::::[choice]Per broker si indicano\: [A8_035_012]{
	=Gli intermediari che agiscono su incarico del cliente# 
	~Gli intermediari che agiscono su incarico della compagnia# 
	~Gli intermediari che agiscono su incarico dell'Autorit&agrave; di vigilanza# 
}

// question: 208077  
$CATEGORY:00A8_035_Intermediazione
::::[choice]Per agente si intende\: [A8_035_011]{
	=Il soggetto che assume stabilmente l&rsquo;incarico di promuovere la conclusione di contratti e che agisce in nome e per conto di una o pi&ugrave; Imprese di Assicurazione, di cui ne ha la rappresentanza# 
	~Il soggetto che assume stabilmente l&rsquo;incarico di promuovere la conclusione di contratti e che agisce per conto di una o pi&ugrave; Imprese di Assicurazione# 
	~Il soggetto che assume stabilmente l&rsquo;incarico di promuovere la conclusione di contratti e che agisce in nome e per conto di una o pi&ugrave; intermediari assicurativi principali# 
}"#
    }
}
