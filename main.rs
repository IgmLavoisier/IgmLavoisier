// prueba lista de listas dd
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn comparar(ca1:&Vec<String>,ca2:&Vec<String>){
    let mut cont:i32=0;
    let m= {
        if ca1.len() > ca2.len() {
            ca1.len() }
            else {
            ca2.len() }
    };
    let mut i1=ca1.into_iter();
    let mut i2=ca2.into_iter();
    for _ in 0..m {
        let p1 = match i1.next() {
            Some(x) => x,
            None => "",
        };

        let p2 = match i2.next() {
            Some(x) => x,
            None => "",
        };

        if p1==p2{
            cont+=1;
        }
        //println!("{:<60} ### {:?}  ", p1 , p2 );
    }

    if (100.0  * cont as f64 /m as f64  )> 50.0
    &&   (100.0  * cont as f64 /m as f64  )< 100.0{
        println!("{} {} {:.3} ", cont , m, 100.0  * cont as f64 /m as f64   );
        let mut i1=ca1.into_iter();
        let mut i2=ca2.into_iter();
        for _ in 0..m {
            let p1 = match i1.next() {
                Some(x) => x,
                None => "",
            };

            let p2 = match i2.next() {
                Some(x) => x,
                None => "",
            };

             if  p1==p2 {
                 println!("{:<60} === {:}  ", p1 , p2 );
             }
             else{
                 println!("{:<60} ### {:}  ", p1 , p2 );
             }
        }
        println!("-----------------------------------------------------------------------------------------------");
    }


    //for li1 in ca1{
    //   for li2 in ca2{
    //      println!("{:90} {}",li1,li2);
    //   }
    //}
}


fn main() -> std::io::Result<()> {
    let st =File::open("../pruebasrust/prueba.hist");
    let mut lista:Vec<String>=Vec::new();

    let file = match st {
        Ok(ffff) => ffff,
        Err(err) => panic!("no existe fichero {} ", err),
    };
    let contenido = BufReader::new(file);

    let mut paque:Vec<Vec<String>>=Vec::new();
    for linea in contenido.lines(){
       let lin = match linea {
           Ok(fff) => fff,
           Err(_fff) => panic!("horror"),
       };
       if lin.contains("@@@") {
           paque.push(lista);
           lista=Vec::new();
           println!("tomaaaa");  }
       lista.push( lin );
    }

    for pa1 in &paque{ 
       for pa2 in &paque{ 
           comparar(pa1,pa2);
          //for linea in pac{ 
              //println!("{}", linea); 
          //}
       }
    }

    Ok(())
}


