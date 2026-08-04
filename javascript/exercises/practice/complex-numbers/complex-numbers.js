//
// Implementation of Complex Number operations in JavaScript
//

export class ComplexNumber {
  constructor(real = 0, imag = 0) {
    this.real = Object.is(real, -0) ? 0 : real;
    this.imag = Object.is(imag, -0) ? 0 : imag;
  }

  add(other) {
    return new ComplexNumber(
      this.real + other.real,
      this.imag + other.imag
    );
  }

  sub(other) {
    return new ComplexNumber(
      this.real - other.real,
      this.imag - other.imag
    );
  }

  div(other) {
    const denom = other.real * other.real + other.imag * other.imag;
    return new ComplexNumber(
      (this.real * other.real + this.imag * other.imag) / denom,
      (this.imag * other.real - this.real * other.imag) / denom
    );
  }

  mul(other) {
    return new ComplexNumber(
      this.real * other.real - this.imag * other.imag,
      this.real * other.imag + this.imag * other.real
    );
  }

  get abs() {
    return Math.sqrt(this.real * this.real + this.imag * this.imag);
  }

  get conj() {
    return new ComplexNumber(this.real, this.imag === 0 ? 0 : -this.imag);
  }

  get exp() {
    // e^(a + bi) = e^a * (cos(b) + i sin(b))
    const expReal = Math.exp(this.real);
    return new ComplexNumber(
      expReal * Math.cos(this.imag),
      expReal * Math.sin(this.imag)
    );
  }
}
