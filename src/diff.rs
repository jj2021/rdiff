pub fn lcs(str1:&str, str2:&str) -> i32 {
    //println!("diff {} and {}", str1, str2);

    // if str1 == "" or str2 == "" return 0
    if str1.is_empty() || str2.is_empty() {
        return 0;
    }

    let str1_vec: Vec<char> = str1.chars().collect();
    let str2_vec: Vec<char> = str2.chars().collect();

    let str1_len = str1.len();
    let str2_len = str2.len();

    // remove last character from string for next iteration
    let str1_next = &str1_vec[..str1_len-1].iter().collect::<String>();
    let str2_next = &str2_vec[..str2_len-1].iter().collect::<String>();

    // if char1 == char2 return 1 + lcs(str1[..len-1], str2[..len-1])
    if str1_vec[str1_len-1] == str2_vec[str2_len-1] {
        return 1 + lcs(str1_next,str2_next);
    } else { // return max( lcs(str1,str2[..len-1]), lcs(str1[..len-1], str2) )
        let a = lcs(str1_next, str2);
        let b = lcs(str1, str2_next);
        return if a > b {a} else {b};
    }
}

pub fn ses(str1:&str, str2:&str) {
    let edits: Vec<(i32,char)> = Vec::new();

    let list = ses_with_vec(str1, str2, edits);

    for edit in list {
        if edit.0 == 1 {
            println!("{} {}", "-", edit.1);
        } else if edit.0 == 2 {
            println!("{} {}", "+", edit.1);
        } else {
            println!("{} {}", " ", edit.1);
        }
    }
}

pub fn ses_with_vec(str1:&str, str2:&str, mut edits: Vec<(i32,char)>) -> Vec<(i32,char)>{
    //println!("diff {} and {}", str1, str2);

    // if str1 == "" or str2 == "" return 0
    if str1.is_empty() {
        for c in str2.chars() {
            edits.push((2,c));
        }
        return edits;
    }

    if str2.is_empty() {
        for c in str1.chars() {
            edits.push((1,c));
        }
        return edits;
    }

    let str1_vec: Vec<char> = str1.chars().collect();
    let str2_vec: Vec<char> = str2.chars().collect();

    let str1_len = str1.len();
    let str2_len = str2.len();

    // remove last character from string for next iteration
    let str1_next = &str1_vec[..str1_len-1].iter().collect::<String>();
    let str2_next = &str2_vec[..str2_len-1].iter().collect::<String>();

    // if char1 == char2 return 1 + lcs(str1[..len-1], str2[..len-1])
    if str1_vec[str1_len-1] == str2_vec[str2_len-1] {
        let mut sub_edits = ses_with_vec(str1_next,str2_next,edits);
        sub_edits.push((3,str1_vec[str1_len-1]));
        return sub_edits;
    } else { // return max( lcs(str1,str2[..len-1]), lcs(str1[..len-1], str2) )
        let editsa = edits.to_vec();
        let editsb = edits.to_vec();
        let mut a = ses_with_vec(str1_next, str2, editsa);
        let mut b = ses_with_vec(str1, str2_next, editsb);
        return if a.len() < b.len() {a.push((1,str1_vec[str1_len-1])); a} else {b.push((2,str2_vec[str2_len-1])); b};
    }
}
