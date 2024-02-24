sealed interface MyEnumWithManyArguments {
    companion object {
        fun createDefault(): MyEnumWithManyArguments = MyEnumWithManyArgumentsOption1(
            0UL,
            0UL,
            0UL,
        )
    }
}

data class MyEnumWithManyArgumentsOption1(
    val p0: ULong,
    val p1: ULong,
    val p2: ULong,
) : MyEnumWithManyArguments

data class MyEnumWithManyArgumentsOption2(
    val name: String,
) : MyEnumWithManyArguments

object MyEnumWithManyArgumentsOption3 : MyEnumWithManyArguments

object MyEnumWithManyArgumentsOption4 : MyEnumWithManyArguments
