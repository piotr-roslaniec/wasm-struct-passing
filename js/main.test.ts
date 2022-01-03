import { MyStruct, process_structs } from "wasm-struct-passing";

const structs = [new MyStruct(1), new MyStruct(2)];

describe("wasm-struct-passing", () => {
  it("process_structs", () => {
    const structsAsBytes = structs.map((s) => s.toBytes());
    const result = process_structs(structsAsBytes);
    expect(result).toEqual(3);
  });
});
