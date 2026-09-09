use std::io;

fn main() {
    let mut express_val: i32 = 0;
    let mut shipping_cost: f64 = 0.0;
    let dis2: f64 = 10.0;
    let dis3: f64 = 20.0;
    println!("Enter weight");
    let mut line = String::new();	
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let weight: f64 = line.trim().parse().expect("Please type a numbxer");
	line.clear();

    println!("Enter destination, 1=domestic, 2 =international");
    io::stdin()
	.read_line(&mut line)
        .expect("Failed to read line");
    let destination: i32 = line.trim().parse().expect("Please type a number");
	line.clear();

    println!("express Y or N");
        io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let express_line = String::new();
	line.clear();

    println!("Define which number tier you are, 1, 2, or 3");
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let tier: i32 = line.trim().parse().expect("Please type a number");
	line.clear();

    if weight <= 1.0 {
        shipping_cost += 5.0;
    }
    if weight > 1.0 && weight <= 5.0 {
        shipping_cost += 12.0;
    }
    if weight > 5.0 && weight <= 20.0 {
        shipping_cost += 25.0;
    } else {
        shipping_cost += 50.0;
    }

	// Get user input for if they are using expres

    if destination == 1 {
    	if express_line == "Y" {
        	express_val += 1;
	}
	        if express_val == 1 {
        	    shipping_cost *= 2.0;
            	    shipping_cost += 20.0;
        } else {
            express_val = 0;
        }
        if tier == 1 {
            shipping_cost += 0.0;
        }
        if tier == 2 {
            let mut discounted_price = shipping_cost * (dis2 / 100.0);
            shipping_cost -= discounted_price;
        }
        if tier == 3 {
            let mut discounted_price = shipping_cost * (dis3 / 100.0);
            shipping_cost -= discounted_price;
	}
	println!("Shipping Cost (shipping_cost):{shipping_cost}");
    }


    if destination == 2 {
        if express_line == "Y" {
            express_val += 1;
        }

	        if express_val == 1 {
        	    shipping_cost *= 2.0;
            	    shipping_cost += 20.0;
        } else {
            express_val = 0;
        }
        if tier == 1 {
            shipping_cost += 0.0;
        }
        if tier == 2 {
            let mut discounted_price = shipping_cost * (dis2 / 100.0);
            shipping_cost -= discounted_price;
        }
        if tier == 3 {
            let mut discounted_price = shipping_cost * (dis3 / 100.0);
            shipping_cost -= discounted_price;
        }
        println!("Shipping Cost (shipping_cost):{shipping_cost}");
    }

}
