import { MyStruct, process_structs, process_map } from "wasm-struct-passing";

const structs = [new MyStruct(1), new MyStruct(2)];

describe("wasm-struct-passing", () => {
  it("process_structs", () => {
    const structsAsBytes = structs.map((s) => s.toBytes());
    const result = process_structs(structsAsBytes);
    expect(result).toEqual(3);
  });

  it("process_map", () => {
    const map = {
      "01": structs[0],
      "02": structs[0],
      "03": structs[0],
    };
    const result = process_map(map);
    expect(result).toEqual(12);
  });
});
