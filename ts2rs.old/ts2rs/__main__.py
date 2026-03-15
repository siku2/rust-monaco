from ts2rs.js_namespace import JsNamespace

if __name__ == "__main__":
    with open("monaco.d.ts") as f:
        s = f.read()

    while s:
        c, s = JsNamespace.consume(s)
        with open(f"{c.name}.rs", "w+") as f:
            f.write(c.to_rust())

