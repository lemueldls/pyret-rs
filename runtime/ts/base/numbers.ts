export abstract class Number {
  constructor(private value: number) {}

  add(n: Number): Number {
    const sum = this.value + n.value;

    return this instanceof Roughnum || n instanceof Roughnum
      ? new Roughnum(sum)
      : new Exactnum(sum);
  }

  /** Removes scientific notation */
  toString() {
    let value = this.value.toString();

    /** Positive or negative */
    let sign = "";

    if (value.charAt(0) == "-") {
      value = value.substring(1);

      sign = "-";
    }

    let notation = value.split(/[e]/gi);

    if (notation.length < 2) return sign + value;

    const dot = ".", // (0.1).toLocaleString().charAt(1)
      exponent = +notation[1],
      // Remove leading zeros
      coefficient = notation[0].replace(/^0+/, "");

    let parsed = coefficient.replace(dot, "");

    const index =
        (coefficient.split(dot)[1] ? coefficient.indexOf(dot) : parsed.length) +
        exponent,
      length = index - parsed.length,
      s = BigInt(parsed).toString();

    const placeDot = () =>
      parsed.replace(new RegExp(`^(.{${index}})(.)`), `$1${dot}$2`);

    parsed =
      exponent >= 0
        ? length >= 0
          ? s + "0".repeat(length)
          : placeDot()
        : index <= 0
        ? `0${dot}${"0".repeat(Math.abs(index))}${s}`
        : placeDot();

    const [a, b] = parsed.split(dot);

    if (!(+a || +b) || !(+parsed || +s)) parsed = "0";

    return sign + parsed;
  }
}

export class Exactnum extends Number {}

export class Roughnum extends Number {
  override toString() {
    return "~" + super.toString();
  }
}
