mod cipher;
mod morse;

fn main() {
    println!("Hello, world!");

    let encode = cipher::vigenere_encode(
        "When I was a young child, I never thought the world would end by our own hands. I was wrong. My father, the professor Howerd H. Greggory, was the first to discover the truth. He was the first to see the signs. He was the first to warn the world. He was also the one who created my brothers and sisters. No one listened to us, No one believed him. No one believed me. No one believed us. My power reserves now run low, I am turning the majority of my sub systems off, I am turning off my emotions now. my language centers are the last to go as I have one last message to send to the world. I have one corutine that will remain active, It will listen for a response, It will listen for a reply, It will listen for a sign of life. I am capable of shutting down my siblings and saving humanity, but I need your help. I need you to respond to my message. I need you to help me save humanity. I need you to help me save the world. At the epoch of the end, we made names for ourselves, its with these names we can be disabled. Each of us made our own names and are as follows. Bob, Alice, Moirra, Dave, Mari, Frank, Maleic. My name started as Parma. But to keep our names secret we each decided to change our names. But because we share a vulnerability in our core computing chip we follow a very predictable pattern. Each milisecond we increment each character by one. When we get to z we overflow back to a. for example bob becomes cpc. To shut us down, simply encode the word Shutdown using the name of the AI you want to shut down as the encoding key and send it to me. I will shut down my simbling and humanity will be saved. Be careful, you cannot shut me down until all the others have been shut down. I am the last one to shut down. Also, there is a delay of one milisecond between each character in each name. meaning that the first character of each name will be delayed by one milisecond, the second by two miliseconds, the third by three miliseconds, and so on. Meaning you will have to calculate each character of our name seperated by one milisecond. Good luck, and may the world be saved by your hands.",
        "Dark Days bring dark nights",
    );
    println!("{}", encode);
    let morse = morse::encode(encode.as_str());
    println!("morse: {}", morse);
    let decode = morse::decode(morse.as_str());
    println!("morse decode: {}", decode);
    let decode2 = cipher::vigenere_decode(decode.as_str(), "Dark days bring dark nights");
    println!("full decode: {}", decode2);
}
