data class BasicTypesModel(
    val byte: UByte,
    val someInteger: Int,
    val someLong: Long,
    val someUnsignedInteger: UInt,
    val someUnsignedLong: ULong,
    val someFloatNumber: Float,
    val someDoubleNumber: Double,
    val someBool: Boolean,
    val someString: String,
    val vector: List<String>,
    val generic: LinearTable<Float, Test>,
    val custom: MyModel,
    val optionalString: String?,
    val optionalListString: List<String>?,
    val listOptionalString: List<String?>,
    val listListString: List<List<String>>,
    val optionalF32: Float?,
) {
    companion object {
        fun createDefault(): BasicTypesModel = BasicTypesModel(
            byte = 0.toUByte(),
            someInteger = 0,
            someLong = 0L,
            someUnsignedInteger = 0U,
            someUnsignedLong = 0UL,
            someFloatNumber = 0f,
            someDoubleNumber = 0.0,
            someBool = false,
            someString = "",
            vector = listOf(),
            generic = LinearTable.createDefault<Float, Test>(),
            custom = MyModel.createDefault(),
            optionalString = null,
            optionalListString = null,
            listOptionalString = listOf(),
            listListString = listOf(),
            optionalF32 = null,
        )

        fun readFromBuffers(reader: Long): BasicTypesModel {
            val byte = UByte.readFromBuffers(reader)
            val someInteger = Int.readFromBuffers(reader)
            val someLong = Long.readFromBuffers(reader)
            val someUnsignedInteger = UInt.readFromBuffers(reader)
            val someUnsignedLong = ULong.readFromBuffers(reader)
            val someFloatNumber = Float.readFromBuffers(reader)
            val someDoubleNumber = Double.readFromBuffers(reader)
            val someBool = Boolean.readFromBuffers(reader)
            val someString = String.readFromBuffers(reader)
            val vector = readFromBuffersList(reader) {
                String.readFromBuffers(reader)
            }
            val generic = LinearTable<Float, Test>.readFromBuffers(reader)
            val custom = MyModel.readFromBuffers(reader)
            val optionalString = readFromBuffersOptional(reader) {
                String.readFromBuffers(reader)
            }
            val optionalListString = readFromBuffersOptional(reader) {
                readFromBuffersList(reader) {
                    String.readFromBuffers(reader)
                }
            }
            val listOptionalString = readFromBuffersList(reader) {
                readFromBuffersOptional(reader) {
                    String.readFromBuffers(reader)
                }
            }
            val listListString = readFromBuffersList(reader) {
                readFromBuffersList(reader) {
                    String.readFromBuffers(reader)
                }
            }
            val optionalF32 = readFromBuffersOptional(reader) {
                Float.readFromBuffers(reader)
            }

            return BasicTypesModel(
                byte = byte,
                someInteger = someInteger,
                someLong = someLong,
                someUnsignedInteger = someUnsignedInteger,
                someUnsignedLong = someUnsignedLong,
                someFloatNumber = someFloatNumber,
                someDoubleNumber = someDoubleNumber,
                someBool = someBool,
                someString = someString,
                vector = vector,
                generic = generic,
                custom = custom,
                optionalString = optionalString,
                optionalListString = optionalListString,
                listOptionalString = listOptionalString,
                listListString = listListString,
                optionalF32 = optionalF32,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                UByte.readFromBuffers(reader)
                Int.readFromBuffers(reader)
                Long.readFromBuffers(reader)
                UInt.readFromBuffers(reader)
                ULong.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Double.readFromBuffers(reader)
                Boolean.readFromBuffers(reader)
                String.readFromBuffers(reader)
                readFromBuffersList(reader) {
                    String.readFromBuffers(reader)
                }
                LinearTable<Float, Test>.readFromBuffers(reader)
                MyModel.readFromBuffers(reader)
                readFromBuffersOptional(reader) {
                    String.readFromBuffers(reader)
                }
                readFromBuffersOptional(reader) {
                    readFromBuffersList(reader) {
                        String.readFromBuffers(reader)
                    }
                }
                readFromBuffersList(reader) {
                    readFromBuffersOptional(reader) {
                        String.readFromBuffers(reader)
                    }
                }
                readFromBuffersList(reader) {
                    readFromBuffersList(reader) {
                        String.readFromBuffers(reader)
                    }
                }
                readFromBuffersOptional(reader) {
                    Float.readFromBuffers(reader)
                }
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        byte.writeToBuffers(writer)
        someInteger.writeToBuffers(writer)
        someLong.writeToBuffers(writer)
        someUnsignedInteger.writeToBuffers(writer)
        someUnsignedLong.writeToBuffers(writer)
        someFloatNumber.writeToBuffers(writer)
        someDoubleNumber.writeToBuffers(writer)
        someBool.writeToBuffers(writer)
        someString.writeToBuffers(writer)
        writeToBuffersList(writer, vector) { vectorItem ->
            vectorItem.writeToBuffers(writer)
        }
        generic.writeToBuffers(writer)
        custom.writeToBuffers(writer)
        writeToBuffersOptional(writer, optionalString) { optionalStringItem ->
            optionalStringItem.writeToBuffers(writer)
        }
        writeToBuffersOptional(writer, optionalListString) { optionalListStringItem ->
            writeToBuffersList(writer, optionalListStringItem) { optionalListStringItemItem ->
                optionalListStringItemItem.writeToBuffers(writer)
            }
        }
        writeToBuffersList(writer, listOptionalString) { listOptionalStringItem ->
            writeToBuffersOptional(writer, listOptionalStringItem) { listOptionalStringItemItem ->
                listOptionalStringItemItem.writeToBuffers(writer)
            }
        }
        writeToBuffersList(writer, listListString) { listListStringItem ->
            writeToBuffersList(writer, listListStringItem) { listListStringItemItem ->
                listListStringItemItem.writeToBuffers(writer)
            }
        }
        writeToBuffersOptional(writer, optionalF32) { optionalF32Item ->
            optionalF32Item.writeToBuffers(writer)
        }
    }
}
