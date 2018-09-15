#![allow(dead_code)]


use indexmap::IndexMap;
use rand::prelude::*;

pub struct ImplicationInformation {
    pub literal: u32,
    pub implied_by_vars: Vec<u32>,
    pub implied_by_clause: Vec<u32>,
    pub uniq_implication_point: bool
}

pub struct CNFFormula {
    pub m_finished: bool,
    pub m_clauses: Vec<Vec<u32>>,
    pub m_variables: IndexMap<String, u32>,
    pub m_assignments: IndexMap<String, bool>,
    pub m_decision_level: i32,
    pub m_decision_guesses: IndexMap<u32, i32>,
    pub m_decision_level_implications: Vec<Vec<ImplicationInformation>>,
    pub m_decision_level_assignments: Vec<IndexMap<String, bool>>
}

impl CNFFormula {
    pub fn add_clause(variables: &IndexMap<String, u32>, clause: Vec<String>) -> Vec<u32> {
        let mut literals: Vec<u32> = Vec::new();

        //println!("Clause: {:?}", clause);

        for literal in clause {
            if literal.starts_with("-") {
                literals.push(variables.get(&literal.trim_matches('-').to_owned()).unwrap().clone() << 1 | 1)
            }
            else if !literal.starts_with("0") {
                literals.push(variables.get(&literal).unwrap().clone() << 1)
            }
        }        

        //println!("Literals: {:?}", literals);
        return literals;
    }

    pub fn new(clause_pile: Vec<Vec<String>>) -> CNFFormula {
        let mut clauses: Vec<Vec<u32>> = Vec::new();
        let mut variables: IndexMap<String, u32> = IndexMap::new();

        let mut var_index = 0;

        for clause in clause_pile {
            for literal in clause.clone() {
                if literal.starts_with("-") {
                    if !variables.contains_key(&literal.trim_matches('-').to_owned()) {
                        variables.insert(literal.trim_matches('-').to_owned(), var_index);
                        var_index += 1;
                    }
                }
                else if !literal.starts_with("0") {
                        if !variables.contains_key(&literal) {
                        variables.insert(literal, var_index);
                        var_index += 1;
                    }
                }
            }
            clauses.push(Self::add_clause(&variables, clause));
        }


        println!("Variables: {:?}\n", variables);

 
        return CNFFormula{ m_finished: false,
                           m_clauses: clauses , m_variables: variables, 
                           m_assignments: IndexMap::new(), m_decision_level: -1,
                           m_decision_guesses: IndexMap::new(),
                           m_decision_level_implications: Vec::new(),
                           m_decision_level_assignments: Vec::new() };
    } 

    pub fn lit_to_string(&self, literal: u32) -> String {
        let mut negated = false;
        if literal & 1 != 0 {
            negated = true;
        }

        let mut num = self.m_variables.get_index((literal >> 1) as usize)
                                    .unwrap()
                                    .0
                                    .to_string();

        if negated {
            num.insert(0, '-');
        }

        return num;
    }

    pub fn lit_list_to_strings(&self, literals: Vec<u32>) -> Vec<String> {
        let mut strings = Vec::new();
        for literal in literals {
            let mut negated = false;
            if literal & 1 != 0 {
                negated = true;
            }

            let mut num = self.m_variables.get_index((literal >> 1) as usize)
                                        .unwrap()
                                        .0
                                        .to_string();

            if negated {
                num.insert(0, '-');
            }
            strings.push(num);
        }

        return strings;
    }

    pub fn print_implications(&self, imp_inf: &ImplicationInformation) {
        println!("variable: {:?} ", self.lit_to_string(imp_inf.literal));
        println!("Implied by vars -> {:?}", self.lit_list_to_strings(imp_inf.implied_by_vars.clone()));
        println!("Implied by clause -> {:?}", self.lit_list_to_strings(imp_inf.implied_by_clause.clone()));
    }

    pub fn print_current_level_implications(&self) {
        for imp_info in &self.m_decision_level_implications[self.m_decision_level as usize] {
            self.print_implications(&imp_info);
        }
    }

    pub fn update_partial_clause(partial_clause: &Vec<u32>, current_clause: &Vec<u32>) -> Vec<u32> {
        let mut new_partial_clause = Vec::new();

        for &partial_literal in partial_clause {
            if !current_clause.contains(&(partial_literal ^ 1)) {
                new_partial_clause.push(partial_literal);
            }
        }

        for &literal in current_clause {
            if !partial_clause.contains(&(literal ^ 1)) && !new_partial_clause.contains(&literal){
                new_partial_clause.push(literal);
            }
        }

        return new_partial_clause;
    }

    pub fn make_decision(&mut self) {
        self.m_decision_level += 1;
        self.m_decision_level_implications.push(Vec::new());
        self.m_decision_level_assignments.push(IndexMap::new());

        let mut rng = thread_rng();
        let mut index = rng.gen_range(0, self.m_variables.len());
        while self.m_assignments.contains_key(&self.m_variables.get_index(index)
                                                  .unwrap()
                                                  .0
                                                  .to_string()) {

            index = rng.gen_range(0, self.m_variables.len());
        }
        
        let rnd_bool = rng.gen();

        // Push decision literal.
        if rnd_bool {
            self.m_decision_guesses.insert(*self.m_variables.get_index(index).unwrap().1 << 1, self.m_decision_level);
            println!("Made decision {}", *self.m_variables.get_index(index).unwrap().1 << 1)
        } else {
            self.m_decision_guesses.insert(*self.m_variables.get_index(index).unwrap().1 << 1 | 1, self.m_decision_level);
            println!("Made decision {}", *self.m_variables.get_index(index).unwrap().1 << 1 | 1)
        }

        

        self.m_assignments.insert(self.m_variables.get_index(index)
                                                  .unwrap()
                                                  .0
                                                  .to_string(), rnd_bool);
        
        self.m_decision_level_assignments[self.m_decision_level as usize].insert(self.m_variables.get_index(index)
                                                                            .unwrap()
                                                                            .0
                                                                            .to_string(), rnd_bool);
   
    }

    pub fn make_decision_fake(&mut self, decision: u32, truthval: bool) {
        self.m_decision_level += 1;
        self.m_decision_level_implications.push(Vec::new());
        self.m_decision_level_assignments.push(IndexMap::new());

        // Push decision literal.
        if truthval {
            self.m_decision_guesses.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1, self.m_decision_level);
            println!("Made decision {}", *self.m_variables.get_index(decision as usize).unwrap().1 << 1)
        } else {
            self.m_decision_guesses.insert(*self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1, self.m_decision_level);
            println!("Made decision {}", *self.m_variables.get_index(decision as usize).unwrap().1 << 1 | 1)
        };


        self.m_assignments.insert(self.m_variables.get_index(decision as usize)
                                                    .unwrap()
                                                    .0
                                                    .to_string(), truthval);

        self.m_decision_level_assignments[self.m_decision_level as usize].insert(self.m_variables.get_index(decision as usize)
                                                                    .unwrap()
                                                                    .0
                                                                    .to_string(), truthval);
    }

    pub fn solve(&mut self) -> bool {

        println!("{:?}", self.m_assignments);
        println!("{:?}", self.m_decision_level_assignments);
        //println!("{:?}", self.m_clauses);

        let mut literal_assignments: Vec<u32> = Vec::new();

        // For potential conflict
        let mut conflict = false;
        let mut partial_learned_clause: Vec<u32> = Vec::new();
        let mut sat_count = 0;
        let mut bt_level = -1;

        // UIP var
        let mut propagate_count = 0;

        // Convert m_assignments into literal assignments.
        for (key, value) in &self.m_assignments {
            let mut negate: u32 = 0;
            if !value {
                negate = 1;
            }

            literal_assignments.push(self.m_variables.get(key).unwrap() << 1 | negate);
        }


        // Process clauses.
        for clause in &self.m_clauses {
            let mut free_literals: Vec<u32> = Vec::new();
            let mut implication_literals: Vec<u32> = Vec::new();
            let mut currently_sat = false;
            

            for literal in clause.clone() {
                let mut free = true;          
                for lit_assignment in &literal_assignments {
                    
                    if literal == (lit_assignment ^ 1) {
                        free = false;

                        // If unit propagation can be made, this array contains the contradicting literals 
                        // AKA the ones that imply the unit propagated.
                        implication_literals.push(literal ^ 1);
                    }
                    if literal == *lit_assignment {
                        currently_sat = true;
                    }
                }

                // If literal is free, push to vector.
                if free {
                    free_literals.push(literal);
                }     
            }

            // Now propagate
            if free_literals.len() == 1 && !currently_sat {
                propagate_count += 1;

                let implication_info = ImplicationInformation {
                    literal: free_literals[0],
                    implied_by_vars: implication_literals,
                    implied_by_clause: clause.clone(),
                    uniq_implication_point: false
                };

                self.m_decision_level_implications[self.m_decision_level as usize].push(implication_info);

                let mut negated = false;
                if free_literals[0] & 1 == 0 {
                    negated = true;
                }

                // Insert into assignments, the unit which must be true in order for clause to be true.
                self.m_assignments.insert(self.m_variables.get_index((free_literals[0] >> 1) as usize)
                                            .unwrap()
                                            .0
                                            .to_string(), negated);

                // Also insert into decision level specific assignments.             
                self.m_decision_level_assignments[self.m_decision_level as usize]
                                .insert(self.m_variables.get_index((free_literals[0] >> 1) as usize)
                                .unwrap()
                                .0
                                .to_string(), negated);
            }

            //If there is a conflict, add the clause
            if free_literals.len() == 0 && !currently_sat {
                conflict = true;

                // Start the partial learned clause by cloning the current clause.
                partial_learned_clause = clause.clone();
                println!("Conflict! {:?}", self.lit_list_to_strings(clause.clone()));

                // FIND CLAUSE TO LEARN.
                while let Some(last_impl) = self.m_decision_level_implications[self.m_decision_level as usize].pop() {
                    partial_learned_clause = Self::update_partial_clause(&partial_learned_clause, &last_impl.implied_by_clause);

                    // Count literals at current decision level that are in clause.
                    let mut lit_count = 0;
                    for p_lit in &partial_learned_clause {            
                        if self.m_decision_level_assignments[self.m_decision_level as usize]
                            .contains_key(&(*p_lit >> 2).to_string()) {
                                lit_count += 1;
                            }
                    }

                    // If there is only one literal from the current decision level left in the clause..
                    // backtrack and learn clause.
                    if lit_count == 1 {
                        break;
                    }              
                }

                // FIND BACKTRACK LEVEL (Highest decision level guess in new learned clause)
                for guess in self.m_decision_guesses.iter() {
                    if partial_learned_clause.contains(&(guess.0 ^ 1)) {
                        bt_level = guess.1.clone();
                    }
                }
                break;
            }

            if currently_sat {
                    sat_count += 1;
            }
        }

        if propagate_count == 1 {
            self.m_decision_level_implications[self.m_decision_level as usize].last_mut().unwrap().uniq_implication_point = true;
            self.print_implications(self.m_decision_level_implications[self.m_decision_level as usize].last().unwrap());
        }

        if conflict {
            if bt_level == -1 {
                println!("UNSAT");
                self.m_finished = true;
                return true;
            }
            // Push conflict clause.
            self.m_clauses.push(partial_learned_clause);

            let mut assignment_removal_count = 0;
            for x in 0..(self.m_decision_level - bt_level) {
                
                // Add the amount of assignments in current decision level.
                assignment_removal_count += self.m_decision_level_assignments[(self.m_decision_level - x) as usize].len();

                // Pop off vectors from current decision level.
                self.m_decision_guesses.pop();
                self.m_decision_level_implications.pop();
                self.m_decision_level_assignments.pop();
            }

            // Set decision level to backtrack level.
            self.m_decision_level = bt_level;

            // Remove amount of assignments calculated from assigments popped off earlier.
            for _x in 0..assignment_removal_count {
                self.m_assignments.pop();
            }

        }

        else if sat_count == self.m_clauses.len() {
            println!("Solution:\n{:?}", self.m_assignments);
            self.m_finished = true;
        }


        return true;
    }
}