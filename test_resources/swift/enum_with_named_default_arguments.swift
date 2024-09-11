enum MyEnumWithNamedArguments: TechPawsBuffersModel {
    case option1(
        /* name */ String,
        /* value */ Float,
        /* bytes */ [UInt8]
    )
    case option2(
        UInt64,
        UInt64,
        UInt64
    )
    case option3
    case option4

    static func createBuffersDefault() -> MyEnumWithNamedArguments {
        return .option1(
            /* name */ "",
            /* value */ 0,
            /* bytes */ []
        )
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let caseValue = bytesReader.readUInt32()

        switch caseValue {
        case 0:
            let name = String.readFromBuffers(bytesReader)
            let value = bytesReader.readFloat()
            let bytes = [UInt8].readFromBuffers(bytesReader)

            return .option1(
                name,
                value,
                bytes
            )
        case 1:
            let p0 = bytesReader.readUInt64()
            let p1 = bytesReader.readUInt64()
            let p2 = bytesReader.readUInt64()

            return .option2(
                p0,
                p1,
                p2
            )
        case 2:
            return .option3
        case 3:
            return .option4
        default:
            fatalError("Invalid value: \(caseValue)")
        }
    }

    static func skipInBuffers(
        _ bytesReader: TechPawsBuffersBytesReader,
        _ count: UInt64
    ) {
        for _ in 1...count {
            let caseValue = bytesReader.readUInt32()

            switch caseValue {
            case 0:
                let _ = String.readFromBuffers(bytesReader)
                let _ = bytesReader.readFloat()
                let _ = [UInt8].readFromBuffers(bytesReader)
            case 1:
                let _ = bytesReader.readUInt64()
                let _ = bytesReader.readUInt64()
                let _ = bytesReader.readUInt64()
            case 2:
                continue
            case 3:
                continue
            default:
                fatalError("Invalid value: \(caseValue)")
            }
        }
    }

    func writeToBuffers(
        _ bytesWriter: TechPawsBuffersBytesWriter
    ) {

        switch self {
        case .option1(
            let name,
            let value,
            let bytes
        ):
            bytesWriter.writeUInt32(0)
            name.writeToBuffers(bytesWriter)
            bytesWriter.writeFloat(value)
            bytes.writeToBuffers(bytesWriter)
        case .option2(
            let p0,
            let p1,
            let p2
        ):
            bytesWriter.writeUInt32(1)
            bytesWriter.writeUInt64(p0)
            bytesWriter.writeUInt64(p1)
            bytesWriter.writeUInt64(p2)
        case .option3:
            bytesWriter.writeUInt32(2)
        case .option4:
            bytesWriter.writeUInt32(3)
        }
    }
}
