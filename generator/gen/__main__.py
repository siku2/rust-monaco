from gen import JsNamespace

if __name__ == "__main__":
    with open("input.d.ts") as fp:
        s = fp.read()

    c, _ = JsNamespace.consume(s)
    print(c.to_rust())
