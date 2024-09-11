struct BasicTypesModel: TechPawsBuffersModel {
    let byte: UInt8
    let someInteger: Int32
    let someLong: Int64
    let someUnsigedInteger: UInt32
    let someUnsigedLong: UInt32
    let someFloatNumber: Float
    let someDoubleNumber: Double
    let someBool: Bool
    let someString: String
    let vector: [String]
    let generic: LinearTable<Float, Test>
    let custom: MyModel
    let optionalString: String?
    let optionalF32: Float?

    static func createBuffersDefault() -> Self {
        return BasicTypesModel(
            byte: 0,
            someInteger: 0,
            someLong: 0,
            someUnsigedInteger: 0,
            someUnsigedLong: 0,
            someFloatNumber: 0,
            someDoubleNumber: 0,
            someBool: false,
            someString: "",
            vector: [],
            generic: LinearTable<Float, Test>.createBuffersDefault(),
            custom: MyModel.createBuffersDefault(),
            optionalString: nil,
            optionalF32: nil
        )
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let byte = bytesReader.readUInt8()
        let someInteger = bytesReader.readInt32()
        let someLong = bytesReader.readInt64()
        let someUnsigedInteger = bytesReader.readUInt32()
        let someUnsigedLong = bytesReader.readUInt32()
        let someFloatNumber = bytesReader.readFloat()
        let someDoubleNumber = bytesReader.readDouble()
        let someBool = bytesReader.readBool()
        let someString = String.readFromBuffers(bytesReader)
        let vector = [String].readFromBuffers(bytesReader)
        let generic = LinearTable<Float, Test>.readFromBuffers(bytesReader)
        let custom = MyModel.readFromBuffers(bytesReader)
        let optionalString = String?.readFromBuffers(bytesReader)
        let optionalF32 = Float?.readFromBuffers(bytesReader)

        return BasicTypesModel(
            byte: byte,
            someInteger: someInteger,
            someLong: someLong,
            someUnsigedInteger: someUnsigedInteger,
            someUnsigedLong: someUnsigedLong,
            someFloatNumber: someFloatNumber,
            someDoubleNumber: someDoubleNumber,
            someBool: someBool,
            someString: someString,
            vector: vector,
            generic: generic,
            custom: custom,
            optionalString: optionalString,
            optionalF32: optionalF32
        )
    }

    static func skipInBuffers(
        _ bytesReader: TechPawsBuffersBytesReader,
        _ count: UInt64
    ) {
        for _ in 1...count {
            let _ = bytesReader.readUInt8()
            let _ = bytesReader.readInt32()
            let _ = bytesReader.readInt64()
            let _ = bytesReader.readUInt32()
            let _ = bytesReader.readUInt32()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readDouble()
            let _ = bytesReader.readBool()
            let _ = String.readFromBuffers(bytesReader)
            let _ = [String].readFromBuffers(bytesReader)
            let _ = LinearTable<Float, Test>.readFromBuffers(bytesReader)
            let _ = MyModel.readFromBuffers(bytesReader)
            let _ = String?.readFromBuffers(bytesReader)
            let _ = Float?.readFromBuffers(bytesReader)
        }
    }

    func writeToBuffers(
        _ bytesWriter: TechPawsBuffersBytesWriter
    ) {
        bytesWriter.writeUInt8(byte)
        bytesWriter.writeInt32(someInteger)
        bytesWriter.writeInt64(someLong)
        bytesWriter.writeUInt32(someUnsigedInteger)
        bytesWriter.writeUInt32(someUnsigedLong)
        bytesWriter.writeFloat(someFloatNumber)
        bytesWriter.writeDouble(someDoubleNumber)
        bytesWriter.writeBool(someBool)
        someString.writeToBuffers(bytesWriter)
        vector.writeToBuffers(bytesWriter)
        generic.writeToBuffers(bytesWriter)
        custom.writeToBuffers(bytesWriter)
        optionalString.writeToBuffers(bytesWriter)
        optionalF32.writeToBuffers(bytesWriter)
    }
}
