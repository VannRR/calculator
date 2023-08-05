use crate::types::*;

// Define constants for square root function
const D_INIT: Int = 1 << (Int::BITS - 2);
const SEARCH_ITER: u8 = ((Int::BITS + 1) / 2) as u8;

// Calculate the absolute value of a signed integer
fn abs(n: Int) -> Int {
    if n >= 0 {
        n
    } else {
        negate(n)
    }
}

// Calculate the two's complement of a signed integer (equivalent to negating its value)
fn negate(n: Int) -> Int {
    let inverted_bits = !n;
    add(inverted_bits, 1)
}

// Calculate the sum of bit a and b
const fn full_adder(a: Int, b: Int, carry_in: Int) -> (Int, Int) {
    // XOR the inputs to calculate the sum bit
    let xor_1 = a ^ b;
    // XOR the result with the carry-in bit to get the final sum bit
    let sum = xor_1 ^ carry_in;
    // AND the intermediate XOR result with the carry-in bit
    let and_1 = xor_1 & carry_in;
    // AND the inputs to calculate the carry-out bit
    let and_2 = a & b;
    // OR the two intermediate AND results to get the final carry-out bit
    let carry_out = and_1 | and_2;

    // Return the sum and carry-out bits as a tuple
    (sum, carry_out)
}

// Calculate the sum of two signed integers
pub fn add(augend: Int, addend: Int) -> Int {
    // Check for special cases where either input is equal to the maximum or minimum representable value
    if (augend == Int::MAX || addend == Int::MAX) && (augend >= 0 || addend >= 0) {
        // If either input is equal to the maximum value and non-negative, return the maximum value
        return Int::MAX;
    } else if (augend == Int::MIN || addend == Int::MIN) && (augend <= 0 || addend <= 0) {
        // If either input is equal to the minimum value and non-positive, return the minimum value
        return Int::MIN;
    } else if augend == 0 && addend == 0
        || augend == Int::MAX && addend == Int::MIN
        || augend == Int::MIN && addend == Int::MAX
    {
        // If both inputs are 0, or one is the maximum value and the other is the minimum value, return 0
        return 0;
    } else if augend == 0 {
        // If augend is 0, return addend unchanged
        return addend;
    } else if addend == 0 {
        // If addend is 0, return augend unchanged
        return augend;
    }

    // Initialize carry and sum variables to 0
    let mut carry = 0;
    let mut sum: Bint = 0;

    // Iterate over each bit position in the Int type
    for i in 0..Int::BITS {
        // Create a mask to extract the bit at the current position from both inputs
        let mask = 1 << i;
        let a_bit = (augend & mask) >> i;
        let b_bit = (addend & mask) >> i;
        // Call the full_adder function to calculate the sum and carry-out bits for this position
        let (s, c) = full_adder(a_bit, b_bit, carry);

        // OR the sum bit into the sum variable at the appropriate position
        sum |= (s << i) as Bint;

        // Store the carry-out bit for use in the next iteration
        carry = c;
    }

    // Check if the sum is outside the representable range of the Int type
    if sum > Int::MAX as Bint {
        // If the sum is greater than Int::MAX, return Int::MAX
        Int::MAX
    } else if sum < Int::MIN as Bint {
        // If the sum is less than Int::MIN, return Int::MIN
        Int::MIN
    } else {
        // Otherwise, convert the sum back to an Int value and return it
        sum as Int
    }
}

// Calculate the difference between two signed integers
pub fn subtract(minuend: Int, subtrahend: Int) -> Int {
    let negated_subtrahend = negate(subtrahend);
    add(minuend, negated_subtrahend)
}

// Calculate the product of two signed integers
pub fn multiply(factor_a: Int, factor_b: Int) -> Int {
    // Check for special cases where either input is 0 or 1
    if factor_a == 0 || factor_b == 0 {
        // If either input is 0, return 0
        return 0;
    } else if factor_a == 1 {
        // If  factor_a is 1, return the value of factor_b
        return factor_b;
    } else if factor_b == 1 {
        // If factor_b is 1, return the value of factor_a
        return factor_a;
    } else if factor_a == -1 {
        // If factor_a is -1, return the inverse of factor_b
        return negate(factor_b);
    } else if factor_b == -1 {
        // If factor_b is -1, return the inverse of factor_a
        return negate(factor_a);
    }

    // Shadow factor_a as a Bint type
    let mut factor_a = factor_a as Bint;
    // Calculate the absolute value of factor_b and store it as a Uint type
    let mut factor_b_abs = abs(factor_b) as Uint;

    // Initialize product variable to 0
    let mut product: Uint = 0;

    // Iterate over each bit position in the Int type
    for _ in 0..Int::BITS {
        // Check if the least significant bit of factor_b is set
        if factor_b_abs & 1 != 0 {
            // If it is, add factor_a to the product variable
            product = add(product as Int, factor_a as Int) as Uint;

            // Check if product is at the minimum or maximum representable value
            if product as Int == Int::MIN || product as Int == Int::MAX {
                // If it is, exit loop
                break;
            }
        }

        // Check if factor_a is not equal to the minimum and maximum representable value
        if factor_a != Int::MIN as Bint && factor_a != Int::MAX as Bint {
            // Left-shift factor_a by 1 bit
            factor_a <<= 1;
        }
        // Check if factor_a is outside the representable range of the Int type
        if factor_a > Int::MAX as Bint {
            // If it is, set factor_a to the maximum representable value
            factor_a = Int::MAX as Bint;
        } else if factor_a < Int::MIN as Bint {
            // If it is, set factor_a to the minimum representable value
            factor_a = Int::MIN as Bint;
        }
        // right-shift factor_b_abs by 1 bit
        factor_b_abs >>= 1;
    }

    // Check if factor_a and factor_b have different signs
    if (factor_a < 0) != (factor_b < 0) {
        // If they do, product is negative, so return the negation of the absolute value of product
        negate(abs(product as Int))
    } else {
        // Otherwise, product is positive, so return the absolute value of product
        abs(product as Int)
    }
}

// Calculate the quotient of two signed integers
pub fn divide(dividend: Int, divisor: Int) -> Int {
    // Check for special cases where the divisor is 0, 1, or -1
    if divisor == 0 {
        // If divisor is 0, return the maximum representable value
        return Int::MAX;
    } else if divisor == 1 {
        // If divisor is 1, return dividend unchanged
        return dividend;
    } else if divisor == -1 {
        // If divisor is -1, return the negation of dividend
        return negate(dividend);
    } else if dividend == 0 {
        // If dividend is 0, return 0
        return 0;
    } else if dividend == divisor {
        // If dividend is equal to divisor, return 1
        return 1;
    }

    // Initialize quotient and remainder variables to 0
    let mut quotient: Uint = 0;
    let mut remainder: Uint = 0;
    // Calculate the absolute values of dividend and divisor and store them as unsigned integers
    let dividend_abs = abs(dividend) as Uint;
    let divisor_abs = abs(divisor) as Uint;
    // Iterate over each bit position in the Int type
    for i in (0..Int::BITS).rev() {
        // Left-shift remainder by 1 bit and set its least significant bit to
        // the corresponding bit of dividend
        remainder <<= 1;
        remainder |= (dividend_abs >> i) & 1;
        // Check if remainder is greater than or equal to divisor_abs
        if remainder >= divisor_abs {
            // If it is, subtract divisor_abs from remainder
            remainder = subtract(remainder as Int, divisor_abs as Int) as Uint;
            // Set the corresponding bit of quotient to 1
            quotient |= 1 << i;
        }
    }
    // Check if dividend and divisor have different signs
    if (dividend < 0) != (divisor < 0) {
        // If they do, return the negation of quotient
        negate(quotient as Int)
    } else {
        // Otherwise, return the quotient as an Int value
        quotient as Int
    }
}

// Calculate the integer square root of a signed integer
pub fn square_root(b: Int) -> Int {
    // Check for special cases where the input is 0, 1, or -1
    if b == 0 {
        // If b is 0, return 0
        return 0;
    } else if b == 1 {
        // If b is 1, return 1
        return 1;
    } else if b == -1 {
        // If b is -1, return -1
        return -1;
    }

    // Initialize root variable to 0 and create a mutable copy of the input
    let mut root: Int = 0;
    let mut x: Int = b;
    // Calculate the initial value of d as 2^(n-2), where n is the number of bits in the Int type
    let mut d: Int = D_INIT;

    // Perform (Int::BITS + 1) / 2 iterations of the loop
    for _ in 0..SEARCH_ITER {
        // Calculate a temporary value by adding root and d
        let a: Int = add(root, d);
        // Check if x is greater than or equal to a
        if x >= a {
            // If it is, subtract a from x
            x = subtract(x, a);
            // Update root by right-shifting it by 1 bit and adding d
            root = add(root >> 1, d);
        } else {
            // Otherwise, simply right-shift root by 1 bit
            root >>= 1;
        }
        // Right-shift d by 2 bits
        d >>= 2;
    }
    // Return the final value of root as the result
    root
}

// Calculate base raised to the power of exponent
pub fn power(mut base: Int, mut exponent: Int) -> Int {
    // Check for special cases where base is 0 or exponent is 0 or 1
    if base == 0 {
        // If base is 0, return 0
        return 0;
    } else if exponent == 0 {
        // If exponent is 0, return 1
        return 1;
    } else if exponent == 1 {
        // If exponent is 1, return base unchanged
        return base;
    }

    // Initialize result variable to 1
    let mut result = 1;
    // Loop until exponent is 0
    while exponent > 0 {
        // Check if the least significant bit of exponent is set
        if (exponent & 1) == 1 {
            // If it is, multiply result by base
            result = multiply(result, base);

            // Check if the result has reached the maximum or minimum representable value
            if result == Int::MAX || result == Int::MIN {
                // If so, return it immediately without performing any further calculations
                return result;
            }
        }
        // Square base by multiplying it by itself
        base = multiply(base, base);
        // Right-shift exponent by 1 bit
        exponent >>= 1;
    }
    // Return the final value of result as the result
    result
}
