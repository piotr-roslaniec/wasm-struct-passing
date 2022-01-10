import {
  MyStruct,
  process_structs,
  process_structs_as_bytes,
  process_map,
  process_map_of_struct_bytes,
} from "wasm-struct-passing";

describe("wasm-struct-passing", () => {
  it("process_structs", () => {
    // Passing structs as plain JS objects
    const structs = [{ value: 1 }, { value: 2 }];

    const result = process_structs(structs);
    expect(result).toEqual(3);
  });

  it("process_structs_as_bytes", () => {
    const structs = [new MyStruct(1).toBytes(), new MyStruct(2).toBytes()];

    const result = process_structs_as_bytes(structs);
    expect(result).toEqual(3);
  });

  it("process_map", () => {
    const map = {
      "01": 1,
      "02": 2,
      "03": 3,
    };

    const result = process_map(map);
    expect(result).toEqual(6);
  });

  it("process_map_of_struct_bytes", () => {
    const map = {
      "01": new MyStruct(1).toBytes(),
      "02": new MyStruct(2).toBytes(),
      "03": new MyStruct(3).toBytes(),
    };

    const result = process_map_of_struct_bytes(map);
    expect(result).toEqual(6);
  });
});
