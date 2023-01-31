use md5;
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::thread;
use std::time::Duration;

//MD5 Challenge JSON from server
#[derive(Debug, Serialize, Deserialize)]
pub struct MD5ChallengeEnum {
    pub MD5HashCash :MD5HashCashInput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5ChallengeStruct {
    pub Challenge: MD5ChallengeEnum,
}
//MD5 Challenge JSON from server

// MD5 Challenge Response JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct MD5ChallengeEnumResponse2 {
    pub MD5HashCash :MD5HashCashOutput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5ChallengeEnumResponse {
    pub answer :MD5ChallengeEnumResponse2,
    pub next_target:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5ChallengeStructResponse {
    pub ChallengeResult: MD5ChallengeEnumResponse,
}
// MD5 Challenge Response JSON
pub trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashChallenge (MD5HashCashInput);

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

impl Challenge for MD5HashCashChallenge {
    /// Données en entrée du challenge
    type Input = MD5HashCashInput;
    /// Données en sortie du challenge
    type Output = MD5HashCashOutput;
    /// Nom du challenge
    fn name() -> String {
        return "MD5HashCash".to_string();
    }
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self {
        return Self (input);
    }
    /// Résout le challenge
    fn solve(&self) -> Self::Output {
        let base: u32 = 2;

        let mut random_number : u32 = rand::thread_rng().gen_range(base.pow(self.0.complexity)..base.pow(self.0.complexity + 1)); // Nombre random
        
        println!("random number : {}", random_number);
        let mut graine : String = format!("{random_number:X}"); //Hexadecimal du nombre random
        let borrowed_string : &str = &self.0.message; // Message
    
        let mut bits : String = "".to_owned();
        let borrowed_bits : &str = "0";
        
        while bits.chars().count() + graine.chars().count() < 16 {
            bits.push_str(borrowed_bits);
        }
        bits.push_str(&graine);
       

        println!("hexa du nombre random : {}", graine);
        println!("seed : {}", bits);
        println!("seed size : {}", bits.chars().count());

        
        let mut seedfinalstring : String = "0b".to_owned();
        seedfinalstring.push_str(&graine);
        println!("{:?}",graine.as_bytes());
        let without_prefix = seedfinalstring.trim_start_matches("0b");
        let seedfinal = u64::from_str_radix(&without_prefix, 16).unwrap();
        println!("seed u64 : {}", seedfinal);
        
        bits.push_str(&borrowed_string);
        
        let digest = md5::compute(bits.as_bytes());
        let hash : String = format!("{:?}", digest);

        
        return Self::Output{seed:seedfinal,hashcode:hash.to_ascii_uppercase()};
    }
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool {
        if answer.seed > 16 {
            return false; 
        }
        else {
            return true;
        }
    }
}

//MonstrousMaze Challenge JSON from server
#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeChallengeEnum {
    pub MonstrousMaze :MonstrousMazeInput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeChallengeStruct {
    pub Challenge: MonstrousMazeChallengeEnum,
}
//MonstrousMaze Challenge JSON from server

// MD5 Challenge Response JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeEnumResponse2 {
    pub MonstrousMaze :MonstrousMazeOutput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeEnumResponse {
    pub answer :MonstrousMazeEnumResponse2,
    pub next_target:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeStructResponse {
    pub ChallengeResult: MonstrousMazeEnumResponse,
}
// MD5 Challenge Response JSON

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeChallenge (MonstrousMazeInput);

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeOutput {
    pub path: String,
}

fn toTheExit(state : &mut Vec<Vec<char>>, parcours : &mut String, position : &mut [usize;2],direction : char) -> String {
    //Savoir combien de chemins possibles
    let mut possibilities = 0;
    if position[1] > 0 {
        if state[position[0]][position[1] - 1] != '#' {
            possibilities += 1;
        }
    }
    if position[1] < state[0].len() - 1  {
        if state[position[0]][position[1] + 1] != '#' {
            possibilities += 1;
        }
    }
    if position[0] > 0 {
        if state[position[0]-1][position[1]] != '#'  {
            possibilities += 1;
        }
    }
    if position[0] < state[0].len() - 1 {
        if state[position[0]+1][position[1]] != '#' {
            possibilities += 1;
        }
    }
    //Entrons
    if state[position[0]][position[1]] != 'X' {
        if state[position[0]][position[1]] == '#' { // Si la position actuelle est un mur
            let mut parcours = &mut parcours[..parcours.len()-2].to_string();
            let b: u8 = parcours.as_bytes()[parcours.len()-1];
            let c: char = b as char;
            toTheExit(state,parcours,position,c);
        }
        else if possibilities == 0 { // Si il y a 0 chemins
            state[position[0]][position[1]] = '#';
            let a = parcours.chars().last().unwrap();
            if a == '>' {
                position[1] = position[1] - 1;
            }
            else if a == '<' {
                position[1] = position[1] + 1;
            }
            else if a == '^' {
                position[0] = position[0] - 1;
            }
            else if a == 'v' {
                position[0] = position[0] + 1;
            }
            
            let mut parcours = &mut parcours[..parcours.len()-2].to_string();
            let b: u8 = parcours.as_bytes()[parcours.len()-1];
            let c: char = b as char;
            toTheExit(state,parcours,position,c);
        }
        else { // Sinon
            if direction == '<' &&  position[1] > 0 {
                if state[position[0]][position[1] - 1] != '#' {
                    if possibilities == 1 {
                        state[position[0]][position[1]] = '#';
                    }
                    else {
                        state[position[0]][position[1]] = 'P';
                    }
                    position[1] -= 1;
                    parcours.push_str("<");
                    toTheExit(state,parcours,position,'<');
                }
                else {
                    let random_number : u8 = rand::thread_rng().gen_range(0..3); // Nombre random
                    let directions = ['>','^','v'];
                    let letsgo = directions[usize::from(random_number)];
                    toTheExit(state,parcours,position,letsgo);
                }
            }
            else if direction == '>' && position[1] < state[0].len() - 1 {
                if state[position[0]][position[1] + 1] != '#' {
                    if possibilities == 1 {
                        state[position[0]][position[1]] = '#';
                    }
                    else {
                        state[position[0]][position[1]] = 'P';
                    }
                    position[1] += 1;
                    parcours.push_str(">");
                    toTheExit(state,parcours,position,'>');
                }
                else {
                    let random_number : u8 = rand::thread_rng().gen_range(0..3); // Nombre random
                    let directions = ['<','^','v'];
                    let letsgo = directions[usize::from(random_number)];
                    toTheExit(state,parcours,position,letsgo);
                }
            }
            else if direction == '^' && position[0] > 0{
                if state[position[0]-1][position[1]] != '#' {
                    if possibilities == 1 {
                        state[position[0]][position[1]] = '#';
                    }
                    else {
                        state[position[0]][position[1]] = 'P';
                    }
                    position[0] -= 1;
                    parcours.push_str("^");
                    toTheExit(state,parcours,position,'^');
                }
                else {
                    let random_number : u8 = rand::thread_rng().gen_range(0..3); // Nombre random
                    let directions = ['<','>','v'];
                    let letsgo = directions[usize::from(random_number)];
                    toTheExit(state,parcours,position,letsgo);
                }
            }
            else if direction == 'v' && position[0] < state[0].len() - 1 {
                if state[position[0]+1][position[1]] != '#'{
                    if possibilities == 1 {
                        state[position[0]][position[1]] = '#';
                    }
                    else {
                        state[position[0]][position[1]] = 'P';
                    }
                    position[0] += 1;
                    parcours.push_str("v");
                    toTheExit(state,parcours,position,'v');
                }
                else {
                    let random_number : u8 = rand::thread_rng().gen_range(0..3); // Nombre random
                    let directions = ['<','>','^'];
                    let letsgo = directions[usize::from(random_number)];
                    toTheExit(state,parcours,position,letsgo);
                }
            }
        }
        
    }
    
    return parcours.to_string();
}

impl Challenge for MonstrousMazeChallenge {
    /// Données en entrée du challenge
    type Input = MonstrousMazeInput;
    /// Données en sortie du challenge
    type Output = MonstrousMazeOutput;
    /// Nom du challenge
    fn name() -> String {
        return "MonstrousMaze".to_string();
    }
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self {
        return Self (input);
    }
    /// Résout le challenge
    fn solve(&self) -> Self::Output {
        let mut largeur = 0;
        let mut longueur = 0;
        let mut position = [0usize;2];

        //Calculer la largeur
        for n in 0..self.0.grid.chars().count() {
            let b : u8 = self.0.grid.as_bytes()[n];
            let c : char = b as char;
            if c == '\n' {
                largeur += 1;
            }
        }
        largeur+=1;
        //Calculer la longueur
        for n in 0..self.0.grid.chars().count() {
            let b : u8 = self.0.grid.as_bytes()[n];
            let c : char = b as char;
            if c == '\n' {
                break;
            }
            else {
                longueur += 1;
            }
        }
        //Création de liste 2 dimensions et remplissage des données
        let mut state = vec![vec![' '; longueur]; largeur];
        let mut messagesize = 0;
        let mut b : u8 = self.0.grid.as_bytes()[messagesize];
        let mut c : char = b as char;
        for mut n in 0..largeur {
            for mut m in 0..longueur {
                if messagesize < self.0.grid.chars().count() {
                    b = self.0.grid.as_bytes()[messagesize];
                    c = b as char;
                    if c == '\n' {
                        messagesize +=1;
                        b = self.0.grid.as_bytes()[messagesize];
                        c = b as char;
                    }
                    if c == 'I' {
                        position[0] = n;
                        position[1] = m;
                    }
                    state[n][m] = c;
                    messagesize += 1;
                    
                }
            }
        }
        for mut n in 0..state[0].len() {
            for mut m in 0..state[0].len() {
                    print!("{}", state[n][m]);
            }
            print!("{}",'\n');
        }
        
        
        //En chemin vers la sortie !
        //let random_number : u8 = rand::thread_rng().gen_range(0..4); // Nombre random
        //let directions = ['<','>','^','v'];
        //let mut letsgo = directions[usize::from(random_number)];
        let mut parcours = "".to_string();
        
        let result = toTheExit(&mut state, &mut parcours, &mut position, 'v');
        println!("{:?}",result);
        
        return MonstrousMazeOutput {path:result.to_string()};
    }
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool {
        return true;
    }
}
