// On the first day of Christmas, my true love sent to me: A partridge in a pear tree.
// On the second day of Christmas, my true love sent to me: Two turtle doves, And a partridge in a pear tree.
// On the third day of Christmas, my true love sent to me: Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the fourth day of Christmas, my true love sent to me: Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the fifth day of Christmas, my true love sent to me: Five golden rings; Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the sixth day of Christmas, my true love sent to me: Six geese a-laying, Five golden rings; Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the seventh day of Christmas, my true love sent to me: Seven swans a-swimming, Six geese a-laying, Five golden rings; Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the eighth day of Christmas, my true love sent to me: Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings; Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the ninth day of Christmas, my true love sent to me: Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings; Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the tenth day of Christmas, my true love sent to me: Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings; Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree.
// On the eleventh day of Christmas, my true love sent to me: Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings; Four calling birds, Three French hens, Two turtle doves,And a partridge in a pear tree.
// On the twelfth day of Christmas, my true love sent to me: Twelve drummers drumming, Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, And a partridge in a pear tree!
fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seven", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let mut gifts = ["Twelve drummers drumming", "Eleven pipers piping", "Ten lords a-leaping", "Nine ladies dancing", "Eight maids a-milking", "Seven swans a-swimming", "Six geese a-laying", "Five golden rings", "Four calling birds", "Three French hens", "Two turtle doves", "a partridge in a pear tree"];
    
    gifts.reverse();

    let mut index:usize = 1;

    for day in days.iter(){
        print!("On the {} day of Christmas, my true love sent to me: ", day);
        for i in (0..index).rev(){
            print!("{}, ",gifts[i]);
        }
        println!(".");
        index += 1;
    }
}
