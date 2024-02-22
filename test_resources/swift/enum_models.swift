enum MyEnum {
    case idle
    case move(
        /* x */ Double,
        /* y */ Double
    )
    case update(
        Double,
        Double,
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

enum MyEnumWithManyArguments {
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

    static func createDefault() -> MyEnumWithManyArguments {
        return .option1(
            0,
            0,
            0
        )
    }
}

enum MyEnumWithNamedArguments {
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

    static func createDefault() -> MyEnumWithNamedArguments {
        return .option1(
            /* name */ "",
            /* value */ 0,
            /* bytes */ []
        )
    }
}
