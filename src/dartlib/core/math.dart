
class Random {

    bool nextBool() {
        return __MATH_NEXT_BOOL();
    }


    double nextDouble() {
        return __MATH_NEXT_DOUBLE();
    }


    int nextInt(int below) {
        return __MATH_NEXT_INT(below);
    }
}


// Converts x to a double and returns its arc cosine in radians.
//
// Returns a value in the range 0..PI, or NaN if x is outside the range -1..1.
double acos(num x) {
    return __MATH_ACOS(x);
}

// Converts x to a double and returns its arc sine in radians.
//
// Returns a value in the range -PI/2..PI/2, or NaN if x is outside
// the range -1..1.
double asin(num x) {
    return __MATH_ASIN(x);
} 

// Converts x to a double and returns its arc tangent in radians.
//
// Returns a value in the range -PI/2..PI/2, or NaN if x is NaN.
double atan(num x) {
    return __MATH_ATAN(x);
}

// A variant of atan.
//
// Converts both arguments to doubles.
// Returns the angle in radians between the positive x-axis and the
// vector (b,a). The result is in the range -PI..PI.
// If b is positive, this is the same as atan(a/b).
// The result is negative when a is negative (including when a is the
// double -0.0). If a is equal to zero, the vector (b,a) is considered
// parallel to the x-axis, even if b is also equal to zero. The sign
// of b determines the direction of the vector along the x-axis.
// Returns NaN if either argument is NaN.
double atan2(num a, num b) {
    return __MATH_ATAN2(a, b);
}

// Converts radians to a double and returns the cosine of the value.
//
// If radians is not a finite number, the result is NaN.
double cos(num radians) {
    return __MATH_COS(radians);
} 

// Converts x to a double and returns the natural exponent, e, to the power x.
//
// Returns NaN if x is NaN.
double exp(num x ) {
    return __MATH_EXP(x)
} 

// Converts x to a double and returns the natural logarithm of the value.
//
// Returns negative infinity if x is equal to zero. Returns NaN if x is NaN or less than zero.
double log(num x) {
    return __MATH_LOG(x);
}

// Returns the larger of two numbers.
//
// Returns NaN if either argument is NaN. The larger of -0.0 and 0.0 is 0.0.
// If the arguments are otherwise equal (including int and doubles with the
// same mathematical value) then it is unspecified which of the two arguments
// is returned.
//
// TODO, should have generic signature
// T max<T extends num>(T a, T b)
num max(num a, num b) {
    return __MATH_MAX(a, b);
}

// Returns the lesser of two numbers.
//
// Returns NaN if either argument is NaN. The lesser of -0.0 and 0.0 is -0.0.
// If the arguments are otherwise equal (including int and doubles with the
// same mathematical value) then it is unspecified which of the two arguments
// is returned.
//
// TODO, should have generic signature
// T min<T extends num>(T a, T b)
num min(num a, num b) {
    return __MATH_MIN(a, b);
}

// Returns x to the power of exponent.
//
// If x is an int and exponent is a non-negative int, the result is an int,
// otherwise both arguments are converted to doubles first, and the result
// is a double.
//
// For integers, the power is always equal to the mathematical result of x
// to the power exponent, only limited by the available memory.
//
// For doubles, pow(x, y) handles edge cases as follows:
//
//   if y is zero (0.0 or -0.0), the result is always 1.0.
//   if x is 1.0, the result is always 1.0.
//   otherwise, if either x or y is NaN, then the result is NaN.
//   if x is negative (but not -0.0) and y is a finite non-integer, the result is NaN.
//   if x is Infinity and y is negative, the result is 0.0.
//   if x is Infinity and y is positive, the result is Infinity.
//   if x is 0.0 and y is negative, the result is Infinity.
//   if x is 0.0 and y is positive, the result is 0.0.
//   if x is -Infinity or -0.0 and y is an odd integer, then the result is -pow(-x ,y).
//   if x is -Infinity or -0.0 and y is not an odd integer, then the result is the same as pow(-x , y).
//   if y is Infinity and the absolute value of x is less than 1, the result is 0.0.
//   if y is Infinity and x is -1, the result is 1.0.
//   if y is Infinity and the absolute value of x is greater than 1, the result is Infinity.
//   if y is -Infinity, the result is 1/pow(x, Infinity).
//
// This corresponds to the pow function defined in the IEEE Standard 754-2008.
// Notice that the result may overflow. If integers are represented as 64-bit numbers, an integer result may be truncated, and a double result may overflow to positive or negative double.infinity.
num pow(num x, num exponent) {
    return __MATH_POW(x, exponent);
}

// Converts radians to a double and returns the sine of the value.
//
// If radians is not a finite number, the result is NaN.
double sin(num radians) {
    return __MATH_SIN(radians);
}

// Converts x to a double and returns the positive square root of the value.
//
// Returns -0.0 if x is -0.0, and NaN if x is otherwise negative or NaN.
double sqrt(num x) {
    return __MATH_SQRT(x);
}

// Converts radians to a double and returns the tangent of the value.
//
// The tangent function is equivalent to sin(radians)/cos(radians) and may be infinite (positive or negative) when cos(radians) is equal to zero. If radians is not a finite number, the result is NaN.
double tan(num radians) {
    return __MATH_TAN(radians);
}

