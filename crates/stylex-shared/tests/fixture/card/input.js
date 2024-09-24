import style from "@stylexjs/stylex";

const c = style.create({
  base: {
    color: "red",
    borderColor: "blue",
  },
  test: {
    borderColor: "pink",
    padding: "10px",
  },
  wrapper: {
    color: "red",
    borderColor: "pink",
  },
  container: {
    marginLeft: "10px",
    padding: "10px",
  }
});

export default function Card() {
  const { className, style } = style.props(c.base, c.test);

  return (
    <article className={className} style={style}>Card</article>
  );
}