
/**
*   Struct with the state parameters
*/
pub struct State {
    aseq: u64,
    cseq: u64,
    term: u64
}

/**
*   Gestion for the state of a Raft's node .
*/
impl State {
    
    /**
    *   Public constructor for the state.
    *   @constructor {u64} term - Index of the leader term
    *   @constructor {u64} cseq - Index of the last commit sequence
    *   @constructor {u64} aseq - Index of the last order append sequence
    */
    pub fn new(term: u64, cseq: u64, aseq: u64) -> State {
        if cseq > aseq {
            panic!("state/state.rs State::new - aseq must be more than cseq");
        }
        State {term: term, cseq: cseq, aseq: aseq}
    }

    /**
     *  Check if the sequence can be appened. 
     *  It mean the index of the sequence must be more than
     *  the aseq of the node . If not, it mean the sequence
     *  have been already appened.
     *  @params{u64} index - Index of the sequence to check for appened.
     */
    pub fn can_append(&self, index: u64) -> bool {
        if index > self.aseq {
            return true;
        }
        return false;
    }

    /**
     *  Check if the sequence can be commited.
     *  A node need to commit the sequence in an order .
     *  Order define by the leader. 
     *  It need to commit the sequence next to the last commit
     *  cseq + 1
     *  @params {u64} index - Index of the sequence to check for commit.
     */
    pub fn can_commit(&self, index: u64) -> bool {
        if index == self.cseq + 1 {
            return true;
        }
        return false;
    }

    /**
     *  Increast the commit sequence (cseq).
     *  Again a node commit the sequence in an order.
     *  So when a node commit a sequence it need to Increast the cseq
     *  for be able to commit the next one.
     *  @params {u64} index - Index of the sequence to commit.
     */
    pub fn commit(&mut self, index: u64) -> bool {
        if self.can_commit(index) {
            self.cseq += 1;
            return true;
        }
        return false;
    }

    /**
     *  Check if the sequence is the next last order append sequence
     *  A Follower need to answer to the leader when it have succefully
     *  appened a sequence.
     *  For that like the commit the sequence need to be the next one
     *  @params {u64} index - Index of the sequence to append
     */
    pub fn append(&mut self, index: u64) -> bool {
        if index == self.aseq + 1 {
            self.aseq += 1;
            return true;
        }
        return false;
    }
    
    /**
     *  Increast the term of a node
     *  When a node fail an election will begin.
     *  The new leader is elected for a term.
     *  Every nodes needs to increast the term.
     *  @params {u64} new_term - Value of the new term.
     */
    pub fn new_term(&mut self, new_term: u64) -> bool {
        if new_term <= self.term {
            return false;
        }
        self.term = new_term;
        true
    }
}

#[test]
fn test_state_new_1() {
    let aseq = 3;
    let cseq = 2;
    let term = 1;
    let test = State::new(term, cseq, aseq);
    assert_eq!(aseq, test.aseq);
    assert_eq!(cseq, test.cseq);
    assert_eq!(term, test.term);
}

#[test]
#[should_panic]
fn test_state_new_2() {
    let aseq = 1;
    let cseq = 2;
    let term = 3;
    State::new(term, cseq, aseq);
}

#[test]
fn test_state_can_append_1() {
    println!("Should return true with index = self.aseq + 1");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.aseq + 1;
    assert_eq!(test.can_append(index), true);
}

#[test]
fn test_state_can_append_2() {
    println!("Should return true with index > self.aseq + 1");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.aseq + 2;
    assert_eq!(test.can_append(index), true);
}

#[test]
fn test_state_can_append_3() {
    println!("Should return false with index = self.aseq");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.aseq;
    assert_eq!(test.can_append(index), false);
}

#[test]
fn test_state_can_append_4() {
    println!("Should return false with index < self.aseq");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.aseq - 1;
    assert_eq!(test.can_append(index), false);
}

#[test]
fn test_state_can_commit_1() {
    println!("Should return true with index = self.cseq + 1");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.cseq + 1;
    assert_eq!(test.can_commit(index), true);
}

#[test]
fn test_state_can_commit_2() {
    println!("Should return false with index > self.cseq + 1");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.cseq + 2;
    assert_eq!(test.can_commit(index), false);
}

#[test]
fn test_state_can_commit_3() {
    println!("Should return false with index = self.cseq");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.cseq;
    assert_eq!(test.can_commit(index), false);
}

#[test]
fn test_state_can_commit_4() {
    println!("Should return false with index < self.cseq");
    let test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.cseq - 1;
    assert_eq!(test.can_commit(index), false);
}

#[test]
fn test_state_commit_1() {
    println!("Should return true with index = self.cseq + 1");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.cseq + 1;
    assert_eq!(test.commit(index), true);
    assert_eq!(test.cseq, index);
}

#[test]
fn test_state_commit_2() {
    println!("Should return false with index > self.cseq + 1");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let cseq = test.cseq;
    let index = cseq + 2;
    assert_eq!(test.commit(index), false);
    assert_eq!(test.cseq, cseq);
}

#[test]
fn test_state_commit_3() {
    println!("Should return false with index = self.cseq");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let cseq = test.cseq;
    let index = cseq;
    assert_eq!(test.commit(index), false);
    assert_eq!(test.cseq, cseq);
}

#[test]
fn test_state_commit_4() {
    println!("Should return false with index < self.cseq");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let cseq = test.cseq;
    let index = cseq - 1;
    assert_eq!(test.commit(index), false);
    assert_eq!(test.cseq, cseq);
}

#[test]
fn test_state_append_1() {
    println!("Should return true with index = self.aseq + 1");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let index = test.aseq + 1;
    assert_eq!(test.append(index), true);
    assert_eq!(test.aseq, index);
}

#[test]
fn test_state_append_2() {
    println!("Should return false with index > self.aseq");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let aseq = test.aseq;
    let index = aseq + 2;
    assert_eq!(test.append(index), false);
    assert_eq!(test.aseq, aseq);
}

#[test]
fn test_state_append_3() {
    println!("Should return false with index = self.aseq");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let aseq = test.aseq;
    let index = aseq ;
    assert_eq!(test.append(index), false);
    assert_eq!(test.aseq, aseq);
}

#[test]
fn test_state_append_4() {
    println!("Should return false with index < self.aseq");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let aseq = test.aseq;
    let index = aseq  - 1;
    assert_eq!(test.append(index), false);
    assert_eq!(test.aseq, aseq);
}

#[test]
fn test_state_new_term_1() {
    println!("Should return true with term > self.term");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let new_term = test.term + 3;
    assert_eq!(test.new_term(new_term), true);
    assert_eq!(test.term, new_term);
}


#[test]
fn test_state_new_term_2() {
    println!("Should return false with term = self.term");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let term = test.term;
    let new_term = term;
    assert_eq!(test.new_term(new_term), false);
    assert_eq!(test.term, term);
}

#[test]
fn test_state_new_term_3() {
    println!("Should return false with term < self.term");
    let mut test = State {aseq: 10, cseq: 10, term: 1};
    let term = test.term;
    let new_term = term - 1;
    assert_eq!(test.new_term(new_term), false);
    assert_eq!(test.term, term);
}
