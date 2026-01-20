use crate::dns_error::DNSError;
use std::result::Result;
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

// Takes the whole DNS message buffer and the offset to which the
// labels parsing should start.
// We need the whole buffer because the DNS compression algorithm
// gives us a pointer where offset 0 is the beginning of the buffer.
// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.4
//
// return
// * Labels => custom type vec<(usize, String)>
// * usize => Length of the labels section in the buffer in bytes
pub fn labels_from_bytes(buffer: &[u8], mut offset: usize) -> Result<(Labels, usize), DNSError> {
    let mut labels = vec![];

    // Iterator starts where labels are supposed to start
    let mut iter = buffer[offset..].iter().peekable();
    let mut labels_total_size = 0;
    let mut count_size = true;
    loop {
        let Some(size) = iter.next() else {
            return Err(DNSError::LabelParsingError(
                "Expect label size got null.".to_string(),
            ));
        };

        // Handle compression
        if (*size & COMPRESSION_TAG) == COMPRESSION_TAG {
            // The pointer offset is encoded on 14bits
            let byte1 = ((*size & 0b0011_1111) as usize) << 8;
            let byte2 = *iter.next().ok_or(DNSError::LabelParsingError(
                "Expect compression pointer after size got nothing".to_string(),
            ))? as usize;

            offset = byte1 | byte2;
            iter = buffer[offset..].iter().peekable();
            // we add two, a pointer is composed of two octet:
            // * the size on 2 bits
            // * the pointer  on 14 bits
            labels_total_size += 2;
            // From now own, we don't count the size because we retrieve
            // a label from another place. The size is the actual size in
            // the buffer.
            count_size = false;
            continue;
        }

        if count_size {
            labels_total_size += *size as usize + 1;
        }

        let mut label = String::new();
        for _ in 0..*size {
            let byte = iter.next().ok_or(DNSError::LabelParsingError(
                "Expect label char got noting".to_string(),
            ))?;
            label.push(*byte as char);
        }

        labels.push((*size, label));
        let Some(&peek) = iter.peek() else {
            return Err(DNSError::LabelParsingError(
                "Expect either null or a size, got EOL".to_string(),
            ));
        };
        // Label is terminated by a null char
        if *peek == 0 {
            if count_size {
                labels_total_size += 1;
            }
            break;
        }
    }

    Ok((labels, labels_total_size))
}

pub fn get_labels_size(labels: &Labels) -> usize {
    let mut size: usize = 0;
    for (s, _) in labels {
        size += *s as usize + 1
    }

    size as usize + 1
}
