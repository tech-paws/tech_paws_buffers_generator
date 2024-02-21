enum MyEnum {
    case idle
    case move(
        /* x */ Float,
        /* y */ Float
    )
    case update(
        Float,
        Float,
        String
    )

    static func createDefault() -> MyEnum {
        return .idle
    }
}

enum MyEnumWithoutPositions {
    case option1(
        UInt64
    )
    case option2(
        /* name */ String
    )
    case option3
    case option4

    static func createDefault() -> MyEnumWithoutPositions {
        return .option1(
            0
        )
    }
}
