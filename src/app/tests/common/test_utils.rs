/// ---------------------------------------------------------------------------------
/// Provides supporting utility methods for use during testing (test case execution)
/// ---------------------------------------------------------------------------------
/// 

use rand::{RngExt, rng};
use chrono::{NaiveDate, NaiveDateTime, TimeDelta};

const LOREUM_IPSUM: &str = r##"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla mollitia officia at tempora enim lorem praesentium labore. Quas voluptatum quaerat veniam duis exercitation magni ab similique nemo. Nisi anim quisquam adipiscing aliqua aute reprehenderit reprehenderit provident sit id ut ad. Irure sint deleniti neque duis obcaecati cillum laboris duis ad blanditiis dignissimos. Do magnam anim quas similique ipsam voluptate vitae minim. Mollitia dicta mollit ratione amet duis id fugit quas do proident explicabo praesentium nemo quos. Numquam minim culpa enim occaecat aspernatur tempor blanditiis magna lorem ut quisquam. Nostrud aliqua ut architecto enim eius nemo porro ipsa ipsa eiusmod aut et consequat minim. Est duis aut eius excepteur atque. Dolores pariatur tempor irure eius veritatis quis elit ullamco minim id adipisci eos vitae sed. Ullamco voluptate non commodo fugiat. Ipsum cupiditate ratione eius magni veritatis. Aliqua id voluptatum dolores odit vitae veniam tempor nemo cillum nemo. Sunt architecto explicabo modi id ipsum similique ipsum ipsam. Sint et dolore voluptas sequi magni ex quos ipsa nostrud nesciunt nemo veniam fugit. Magnam aspernatur proident reprehenderit similique elit tempor. Sunt beatae ipsa consectetur nisi ea nesciunt qui. Accusamus obcaecati aspernatur ipsum occaecat non sunt nesciunt odit. Quas non non cillum lorem sint amet architecto dolorem neque ipsum.Voluptas nisi aspernatur sequi mollitia ex ipsum anim in iusto officia odio adipisci amet similique. Dolores ut in pariatur ea. Enim sequi elit ullamco modi aut. Id beatae cupiditate quas aute pariatur elit. Sed fugiat aliqua deleniti quaerat nemo enim esse nisi irure porro excepteur reprehenderit. Dicta accusamus mollitia enim quae quos est ex neque. Dolor ea ad anim cupiditate cillum et labore provident culpa magni. Nisi vitae animi molestias quas. Mollitia eiusmod provident mollitia quos adipiscing lorem sequi dolore porro illo laboris voluptatum. Fugiat tempor fugit magni est proident laborum dolor. Aute modi mollitia sequi commodo et dolores laborum. Quaerat quisquam beatae ipsa nisi dolor aspernatur officia labore vero mollitia. Neque beatae id blanditiis duis. Atque mollitia fugit et adipiscing enim amet ex duis quisquam officia nostrud elit laboris neque. Consectetur animi ipsa tempora occaecat veniam voluptate mollit iusto. Accusamus similique magna ullamco officia nesciunt excepteur animi dignissimos odit odit. Quae quis irure magni elit porro. Tempora ad ex odio dignissimos pariatur officia dicta obcaecati aspernatur quia blanditiis. Duis ullamco consequat sint eos ab consequuntur amet ipsa nemo odit veniam aspernatur. Exercitation ipsum nostrud minim molestias illo neque provident mollit est illo dignissimos deserunt in vitae. Culpa ipsam vero lorem neque magna at cillum. Quos dignissimos architecto labore quos cupidatat lorem minim non ea aliquip qui enim tempor duis. Quis magni cupiditate mollit reprehenderit. Sint proident ratione sit dolorem iusto. Pariatur dolorem proident obcaecati consectetur commodo. Voluptate porro at voluptate obcaecati culpa sed quasi elit. Aute ducimus deleniti accusamus corrupti. Sequi voluptas ducimus magna quisquam voluptas ullamco blanditiis voluptate. Iusto nostrud anim est cupiditate aliquip laborum nulla ullamco sed vero enim mollit. Porro officia esse sed ipsa sed. Iusto duis porro quas animi neque quaerat corrupti explicabo beatae. Iusto proident quia deserunt occaecat dolorem. Ullamco fugit veritatis dicta lorem ipsum nulla qui consectetur nostrud deserunt minim enim. Fugit deserunt quae commodo aliquip quis cupidatat non occaecat aut pariatur neque corrupti nulla. Blanditiis aliquip quasi architecto occaecat at ad aute voluptas. Eos inventore neque beatae ex. Ratione cupiditate accusamus est quia quasi. Numquam mollit eiusmod consectetur numquam eos sed eos numquam. Provident odio blanditiis ab ullamco. Modi exercitation excepteur ut sed quasi inventore cillum nostrud laborum aute. Sint obcaecati laborum quas aute commodo commodo lorem. Incididunt ea dignissimos porro pariatur."##;

#[cfg(test)]
pub struct DataGenerator;

#[cfg(test)]
impl DataGenerator{
    ///
    /// Returns a slice of lorem ipsum text, for use in testing
    /// 
    ///
    pub fn get_lorem_ipsum(length: usize) -> String {
        let result = LOREUM_IPSUM;
        return result[0..length].to_string();
    }

    pub fn get_first_name(length: usize) -> String {
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["Albert", "Marie", "Winston", "Abraham", "Cleopatra", "Isaac", "Leonardo", "Florence", "Nikola", "Ada", "Mahatma", "Frida", "Charles", "Rosa", "Wolfgang", "Amelia", "Galileo", "Emily", "Ludwig", "Vincent"]; 
        let result = name_list.get( rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a random name from the prior list

        match result.len() < length {
            true => result.to_string(),
            false => result[0..length].to_string()
        }
    }

    pub fn get_last_name(length: usize) -> String {
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["Einstein", "Curie", "Churchill", "Lincoln", "Philopator", "Newton", "da Vinci", "Nightingale", "Tesla", "Lovelace", "Gandhi", "Kahlo", "Darwin", "Parks", "Mozart", "Earhart", "Galilei", "Dickinson", "Beethoven", "van Gogh"]; 
        let result = name_list.get(rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a random name from the prior list

        match result.len() < length {
            true => result.to_string(),
            false => result[0..length].to_string()
        }
    }

    pub fn get_middle_names(length: usize) -> String {
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["Lemon", "Salomea", "Leonard Spencer", "", "", "", "", "", "", "Augusta King", "Mohandas Karamchand", "", "Robert", "Louise", "Amadeus", "Mary", "", "Elizabeth", "van", "Willem"]; 
        let result = name_list.get(rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a random name from the prior list

        match result.len() < length {
            true => result.to_string(),
            false => result[0..length].to_string()
        }
    }

    pub fn get_phn() -> i64 {
        let mut rng = rng();

        return rng.random_range(9000000000..9999999999) ;
    }

    ///
    /// Returns a random date between Aug 21, 1909 and Augt 17, 2026(ish)... for testing purposes
    /// ref: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html
    /// 
    pub fn get_date() -> NaiveDateTime {
        let mut rng = rng();

        // start with a base date, generate a random number of seconds and add to come up with a new randomized date
        // we're going to use Ethel Caterham (oldest person alive as of Aug 18, 2026)
        let base_date: NaiveDateTime = NaiveDate::from_ymd_opt(1909, 9, 21).unwrap().and_hms_opt(1, 1, 1).unwrap();

        let td = TimeDelta::try_seconds( rng.random_range(0..3723446580)).unwrap(); // why this number? I googled the number of seconds between today and Ethel's birthday

        base_date + td // return Ethel's birtdate + random duration between it and today
    }
    
    ///
    /// Returns a date with a random number of seconds added
    ///
    pub fn add_random_seconds(base_date: NaiveDateTime, lower_bound: i64, upper_bound: i64) -> NaiveDateTime {
        let mut rng = rng();

        let td = TimeDelta::try_seconds( rng.random_range(lower_bound..upper_bound)).unwrap(); 

        base_date + td 
    }

    ///
    /// Returns randomly selected room identifier from a predefined list
    ///
    pub fn get_room_identifier(length: usize) -> String {
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["00-1D", "00-1C", "110", "Exam Room 1", "125", "170B", "E2A", "W2B", "00-2A", "01-3B", "112", "Exam Room 2", "130", "175C", "E3B", "W1A", "205", "L4D"]; 
        let result = name_list.get(rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a item from the prior list

        match result.len() < length {
            true => result.to_string(),
            false => result[0..length].to_string()
        }
    }

    pub fn get_intv_status_description() -> String{
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["New (Unassigned)", "Pending (Assigned)", "In Progress", "On Hold", "Complete", "Archived"]; 
        let result = name_list.get(rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a item from the prior list

        result.to_string()
    }
    
    pub fn get_intv_type_description() -> String{
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["Bandage", "Bloodwork", "CT Scan", "MRI", "Medication", "Port", "Referral", "Surgery", "Suture", "Transfusion", "X-Ray", "Other", "Vitals"]; 
        let result = name_list.get(rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a item from the prior list

        result.to_string()
    }

    pub fn get_intv_detail_type_description() -> String{
        let mut rng = rng();
        let name_list: Vec<&str> = vec!["Height (cm)", "Weight (kg)", "HR", "BP", "RR", "Sp02", "Temp (C)", "PL", "Vitals", "Eye"]; 
        let result = name_list.get(rng.random_range(0..name_list.len()-1 ) ).unwrap(); // return a item from the prior list

        result.to_string()
    }    
}