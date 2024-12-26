struct BasicTypesModel: TechPawsBuffersModel {
    let byte: UInt8
    let someInteger: Int32
    let someLong: Int64
    let someUnsignedInteger: UInt32
    let someUnsignedLong: UInt64
    let someFloatNumber: Float
    let someDoubleNumber: Double
    let someBool: Bool
    let someString: String
    let vector: [String]
    let generic: LinearTable<Float, Test>
    let custom: MyModel
    let optionalString: String?
    let optionalListString: [String]?
    let listOptionalString: [String?]
    let listListString: [[String]]
    let optionalF32: Float?

    static func createBuffersDefault() -> Self {
        return BasicTypesModel(
            byte: 0,
            someInteger: 0,
            someLong: 0,
            someUnsignedInteger: 0,
            someUnsignedLong: 0,
            someFloatNumber: 0,
            someDoubleNumber: 0,
            someBool: false,
            someString: "",
            vector: [],
            generic: LinearTable<Float, Test>.createBuffersDefault(),
            custom: MyModel.createBuffersDefault(),
            optionalString: nil,
            optionalListString: nil,
            listOptionalString: [],
            listListString: [],
            optionalF32: nil
        )
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let byte = bytesReader.readUInt8()
        let someInteger = bytesReader.readInt32()
        let someLong = bytesReader.readInt64()
        let someUnsignedInteger = bytesReader.readUInt32()
        let someUnsignedLong = bytesReader.readUInt64()
        let someFloatNumber = bytesReader.readFloat()
        let someDoubleNumber = bytesReader.readDouble()
        let someBool = bytesReader.readBool()
        let someString = String.readFromBuffers(bytesReader)
        let vector = [String].readFromBuffers(bytesReader)
        let generic = LinearTable<Float, Test>.readFromBuffers(bytesReader)
        let custom = MyModel.readFromBuffers(bytesReader)
        let optionalString = String?.readFromBuffers(bytesReader)
        let optionalListString = [String]?.readFromBuffers(bytesReader)
        let listOptionalString = [String?].readFromBuffers(bytesReader)
        let listListString = [[String]].readFromBuffers(bytesReader)
        let optionalF32 = Float?.readFromBuffers(bytesReader)

        return BasicTypesModel(
            byte: byte,
            someInteger: someInteger,
            someLong: someLong,
            someUnsignedInteger: someUnsignedInteger,
            someUnsignedLong: someUnsignedLong,
            someFloatNumber: someFloatNumber,
            someDoubleNumber: someDoubleNumber,
            someBool: someBool,
            someString: someString,
            vector: vector,
            generic: generic,
            custom: custom,
            optionalString: optionalString,
            optionalListString: optionalListString,
            listOptionalString: listOptionalString,
            listListString: listListString,
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
            let _ = bytesReader.readUInt64()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readDouble()
            let _ = bytesReader.readBool()
            let _ = String.readFromBuffers(bytesReader)
            let _ = [String].readFromBuffers(bytesReader)
            let _ = LinearTable<Float, Test>.readFromBuffers(bytesReader)
            let _ = MyModel.readFromBuffers(bytesReader)
            let _ = String?.readFromBuffers(bytesReader)
            let _ = [String]?.readFromBuffers(bytesReader)
            let _ = [String?].readFromBuffers(bytesReader)
            let _ = [[String]].readFromBuffers(bytesReader)
            let _ = Float?.readFromBuffers(bytesReader)
        }
    }

    func writeToBuffers(
        _ bytesWriter: TechPawsBuffersBytesWriter
    ) {
        bytesWriter.writeUInt8(byte)
        bytesWriter.writeInt32(someInteger)
        bytesWriter.writeInt64(someLong)
        bytesWriter.writeUInt32(someUnsignedInteger)
        bytesWriter.writeUInt64(someUnsignedLong)
        bytesWriter.writeFloat(someFloatNumber)
        bytesWriter.writeDouble(someDoubleNumber)
        bytesWriter.writeBool(someBool)
        someString.writeToBuffers(bytesWriter)
        vector.writeToBuffers(bytesWriter)
        generic.writeToBuffers(bytesWriter)
        custom.writeToBuffers(bytesWriter)
        optionalString.writeToBuffers(bytesWriter)
        optionalListString.writeToBuffers(bytesWriter)
        listOptionalString.writeToBuffers(bytesWriter)
        listListString.writeToBuffers(bytesWriter)
        optionalF32.writeToBuffers(bytesWriter)
    }
}
