import {
  MyStruct,
  process_structs_1,
  process_structs_2,
  process_structs_3,
  process_structs_4,
  process_structs_5,
} from "wasm-struct-passing";

const structs = [new MyStruct(1), new MyStruct(2)];

describe("MyStruct", () => {
  // Throws
  it("process_structs_1", () => {
    const result = process_structs_1(structs);
    expect(result).toEqual(3);
  });

  // Throws
  it("process_structs_2", () => {
    const result = process_structs_2(structs);
    expect(result).toEqual(3);
  });

  // Throws
  it("process_structs_3", () => {
    const result = process_structs_3(structs);
    expect(result).toEqual(3);
  });

  // Throws
  it("process_structs_4", () => {
    const result = process_structs_4(structs);
    expect(result).toEqual(3);
  });

  // Works
  it("process_structs_5", () => {
    const structsAsBytes = structs.map((s) => s.toBytes());
    const result = process_structs_5(structsAsBytes);
    expect(result).toEqual(3);
  });
});
