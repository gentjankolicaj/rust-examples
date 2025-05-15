

pub fn write_flag()->bool{
    //do something
    //return true
    true
}

pub fn return_value()->i32{
    //do something
    
    //return 0
    0
}
#[cfg(test)]
mod integration_tests {
    use crate::{return_value, write_flag};

    #[test]
    fn check_write_flag() {
        assert!(write_flag());
        
    }
    
    #[test]
    fn assert_return_value(){
        assert_eq!(return_value(),0);
    }
}
