enum MyEnumWithManyArguments: TechPawsBuffersModel {
    case option1(
        UInt64,
        UInt64,
        UInt64
    )
    case option2(
        /* name */ String
    )
    case option3
    case option4

    static func createBuffersDefault() -> MyEnumWithManyArguments {
        return .option1(
            0,
            0,
            0
        )
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let caseValue = bytesReader.readUInt32()

        switch caseValue {
        case 0:
            let p0 = bytesReader.readUInt64()
            let p1 = bytesReader.readUInt64()
            let p2 = bytesReader.readUInt64()

            return .option1(
                p0,
                p1,
                p2
            )
        case 1:
            let name = String.readFromBuffers(bytesReader)

            return .option2(
                name
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
                let _ = bytesReader.readUInt64()
                let _ = bytesReader.readUInt64()
                let _ = bytesReader.readUInt64()
            case 1:
                let _ = String.readFromBuffers(bytesReader)
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
            let p0,
            let p1,
            let p2
        ):
            bytesWriter.writeUInt32(0)
            bytesWriter.writeUInt64(p0)
            bytesWriter.writeUInt64(p1)
            bytesWriter.writeUInt64(p2)
        case .option2(
            let name
        ):
            bytesWriter.writeUInt32(1)
            name.writeToBuffers(bytesWriter)
        case .option3:
            bytesWriter.writeUInt32(2)
        case .option4:
            bytesWriter.writeUInt32(3)
        }
    }
}
