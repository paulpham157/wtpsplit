use nnsplit::{NNSplit, Token};

macro_rules! token {
    ($text:expr, $whitespace:expr) => (Token {
        text: String::from($text),
        whitespace: String::from($whitespace) 
    });
}

#[test]
fn it_splits_german_correctly() -> failure::Fallible<()> {
    let splitter = NNSplit::new("de")?;

    let result = splitter.split(vec!["Das ist ein Test Das ist noch ein Test."]);

    assert_eq!(vec![vec![
                vec![
                    token!("Das", " "), 
                    token!("ist", " "),
                    token!("ein", " "),
                    token!("Test", " "),
                ],
                vec![
                    token!("Das", " "), 
                    token!("ist", " "),
                    token!("noch", " "),
                    token!("ein", " "),
                    token!("Test", ""),
                    token!(".", ""),
                ],
            ]
        ], result
    );

    Ok(())
}

#[test]
fn test_it_batches_correctly() -> failure::Fallible<()> {
    let mut splitter = NNSplit::new("de")?;
    splitter.with_batch_size(2);

    let result = splitter.split(vec!["First", "Second", "Third"]);

    assert_eq!(
        vec![
            vec![
                vec![
                    token!("First", ""),
                ]
            ],
            vec![
                vec![
                    token!("Second", ""),
                ]
            ],
            vec![
                vec![
                    token!("Third", ""),
                ]
            ],
        ], result
    );

    Ok(())
}