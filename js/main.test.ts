import { MyStruct } from "wasm-struct-passing";

describe("MyStruct", () => {
  it("works", () => {
    const myStruct = new MyStruct(42);
    myStruct.free();
  });
});
