sealed interface MyEnumWithNamedArguments {
    companion object {
        fun createDefault(): MyEnumWithNamedArguments = MyEnumWithNamedArgumentsOption1(
            name = "",
            value = 0f,
            bytes = listOf(),
        )
    }
}

data class MyEnumWithNamedArgumentsOption1(
    val name: String,
    val value: Float,
    val bytes: List<UByte>,
) : MyEnumWithNamedArguments

data class MyEnumWithNamedArgumentsOption2(
    val p0: ULong,
    val p1: ULong,
    val p2: ULong,
) : MyEnumWithNamedArguments

object MyEnumWithNamedArgumentsOption3 : MyEnumWithNamedArguments

object MyEnumWithNamedArgumentsOption4 : MyEnumWithNamedArguments
