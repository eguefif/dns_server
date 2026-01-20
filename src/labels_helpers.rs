pub type Labels = Vec<(u8, String)>;

const COMPRESSION_TAG: u8 = 0b1100_0000;

pub fn labels_from_string(domain: String) -> Labels {
    let splits = domain.split(".");
    let mut labels = vec![];
    for split in splits {
        labels.push((split.len() as u8, split.to_string()));
    }

    labels
}

pub fn labels_to_bytes(labels: &[(u8, String)]) -> Vec<u8> {
    let mut bytes = vec![];

    for (len, label) in labels {
        bytes.push(*len);
        bytes.extend_from_slice(&label.clone().into_bytes());
    }

    return bytes;
}


pub fn labels_from_bytes(buffer: &[u8], mut offset: usize) -> (Labels, usize) {
    let mut labels = vec![];

    let mut iter = buffer[offset..].iter().peekable();
    let mut labels_size = 0;
    let mut compression = false;
    loop {
        // TODO: Refactor, is there a more idiomatic way?
        let Some(size) = iter.next() else {
            todo!("handle error: early stop")
        };
        if *size == COMPRESSION_TAG {
            offset = *iter.next().unwrap() as usize;
            iter = buffer[offset..].iter().peekable();
            labels_size += 2;
            compression = true;
            continue;
        }
        if !compression {
            labels_size += *size as usize + 1;
        }
        let mut label = String::new();
        for _ in 0..*size {
            let byte = iter.next().unwrap();
            label.push(*byte as char);
        }
        labels.push((*size, label));
        let Some(&peek) = iter.peek() else {
            todo!("Handle error: early stop")
        };
        if *peek == 0 {
            if !compression {
                labels_size += 1;
            }
            break;
        }
    }

    (labels, labels_size)
}

pub fn get_labels_size(labels: &Labels) -> usize {
    let mut size: usize = 0;
    for (s, _) in labels {
        size += *s as usize + 1
    }

    size as usize + 1
}
