use regex::Regex;

const QUESTION_PATTERN: &str = r"::::\[choice\](.*?)(\\:)?\s*\[";
const ANSWER_PATTERN: &str = r"\s*([=~])(.*?)#?\s*$";

pub struct Chunk(pub Vec<String>);

struct ChunkIterator {
    input: String,
    current_position: usize,
}

impl Iterator for ChunkIterator {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = Chunk(
            self.input
                .lines()
                .skip(self.current_position)
                .take_while(|l| l.len() > 0)
                .map(|l| html_escape::decode_html_entities(l).to_string())
                .collect(),
        );

        if chunk.0.len() < 1 {
            return None;
        }

        self.current_position += chunk.0.len() + 1;

        Some(chunk)
    }
}

pub struct Question {
    category: String,
    text: String,
    answers: Vec<String>,
    correct_answer: String,
}

pub fn convert(input_filename: &str, output_filename: &str) -> anyhow::Result<()> {
    let input_content = std::fs::read_to_string(input_filename)?;

    let question_matcher = Regex::new(QUESTION_PATTERN)?;
    let answer_matcher = Regex::new(ANSWER_PATTERN)?;

    let questions = parse_input(&input_content, question_matcher, answer_matcher)?;

    let mut writer = csv::Writer::from_path(output_filename)?;

    for question in questions {
        writer.write_record(
            [
                vec![question.category],
                vec![question.text],
                question.answers,
                vec![question.correct_answer],
            ]
            .concat(),
        )?;
    }

    Ok(())
}

fn parse_input(
    input: &str,
    question_matcher: Regex,
    answer_matcher: Regex,
) -> anyhow::Result<Vec<Question>> {
    let mut chunk_iterator = ChunkIterator {
        input: input.to_string(),
        current_position: 0,
    };

    let mut questions = vec![];

    while let Some(chunk) = chunk_iterator.next() {
        let question = parse_chunk(chunk, question_matcher.clone(), answer_matcher.clone())?;
        questions.push(question);
    }

    Ok(questions)
}

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
        chunk
            .0
            .iter()
            .skip(3)
            .filter(|line| line.len() > 1)
            .map(|line| answer_matcher.captures(line))
            .enumerate()
            .filter_map(|(i, caps)| {
                let captures = caps?;

                if captures.get(1)?.as_str() == "=" {
                    correct_answer = i;
                }

                let answer = captures.get(2)?.as_str().to_string();
                Some(answer)
            })
            .collect()
    };

    let correct_answer_letter = match correct_answer {
        0 => "A",
        1 => "B",
        2 => "C",
        _ => panic!("Invalid correct answer"),
    };

    Ok(Question {
        category,
        text: question,
        answers,
        correct_answer: correct_answer_letter.to_string(),
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

        let mut count = 1;
        while let Some(_) = chunk_iterator.next() {
            count += 1;
        }
        assert_eq!(count, 3);
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
        assert_eq!(question.answers[0], "Falso");
        assert_eq!(
            question.answers[1],
            "Vero, ma solo provato con il pagamento presso i locali dell'intermediario"
        );
        assert_eq!(question.answers[2], "Vero");
        assert_eq!(question.correct_answer, "C");
    }

    #[test]
    fn parse_chunk_with_html_cars() {
        let input = gift_example_with_html_chars();
        let mut chunk_iterator = ChunkIterator {
            input: input.to_string(),
            current_position: 0,
        };

        let chunk = chunk_iterator.next().unwrap();

        let question_matcher = Regex::new(QUESTION_PATTERN).unwrap();
        let answer_matcher = Regex::new(ANSWER_PATTERN).unwrap();

        let question = parse_chunk(chunk, question_matcher, answer_matcher).unwrap();
        assert_eq!(question.text, "Cos'è un ransomware?");
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
}
"#
    }

    fn gift_example_with_html_chars() -> &'static str {
        r#"// question: 207853  
$CATEGORY:B0_109_cyber risk office
::::[choice]Cos'&egrave; un ransomware? [B0_109_10]{
	~Un blocco delle attivit&agrave; a causa di un virus
	=Una richiesta di riscatto a seguito di attacco cyber
	~Un malware specifico per le aziende
}"#
    }
}
