enum MyEnum: TechPawsBuffersModel {
    case idle
    case move(
        /* y */ Double,
        /* x */ Double
    )
    case update(
        String,
        Float,
        Int32,
        Double
    )

    static func createBuffersDefault() -> MyEnum {
        return .idle
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let caseValue = bytesReader.readUInt32()

        switch caseValue {
        case 3:
            return .idle
        case 1:
            let y = bytesReader.readDouble()
            let x = bytesReader.readDouble()

            return .move(
                y,
                x
            )
        case 2:
            let p1 = String.readFromBuffers(bytesReader)
            let p2 = bytesReader.readFloat()
            let p3 = bytesReader.readInt32()
            let p8 = bytesReader.readDouble()

            return .update(
                p1,
                p2,
                p3,
                p8
            )
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
            case 3:
                continue
            case 1:
                let _ = bytesReader.readDouble()
                let _ = bytesReader.readDouble()
            case 2:
                let _ = String.readFromBuffers(bytesReader)
                let _ = bytesReader.readFloat()
                let _ = bytesReader.readInt32()
                let _ = bytesReader.readDouble()
            default:
                fatalError("Invalid value: \(caseValue)")
            }
        }
    }

    func writeToBuffers(
        _ bytesWriter: TechPawsBuffersBytesWriter
    ) {

        switch self {
        case .idle:
            bytesWriter.writeUInt32(3)
        case .move(
            let y,
            let x
        ):
            bytesWriter.writeUInt32(1)
            bytesWriter.writeDouble(y)
            bytesWriter.writeDouble(x)
        case .update(
            let p1,
            let p2,
            let p3,
            let p8
        ):
            bytesWriter.writeUInt32(2)
            p1.writeToBuffers(bytesWriter)
            bytesWriter.writeFloat(p2)
            bytesWriter.writeInt32(p3)
            bytesWriter.writeDouble(p8)
        }
    }
}
