constexpr int x = 42;

/*===
Declaration
    DeclarationSpecifier
        StorageClassSpecifier Constexpr
    DeclarationSpecifier
        TypeSpecifier Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "x"
        Initializer
            Expression
                Constant
                    Integer "42"
                        IntegerBase Decimal
                        IntegerSuffix false false
                            IntegerSize Int
===*/
