pub trait State {
    fn is_final(&self) -> bool;
}

Accept!(AndAcc);
Accept!(AssignAcc);
Accept!(BiggerAcc);
Accept!(BinaryAcc);
Accept!(CaretAcc);
Accept!(Colon);
Accept!(Comma);
Accept!(SlashAcc);
Accept!(DecimalAcc);
Accept!(DecimalDigitsAcc);
Accept!(DecimalExponentAcc);
Accept!(DecimalExponentSignedAcc);
Accept!(DotPart);
Accept!(DoubleString);
Accept!(ExclamationAcc);
Accept!(HELL);
Accept!(HexAcc);
Accept!(Identifier);
Accept!(LCurly);
Accept!(LesserAcc);
Accept!(LineTerminator);
Accept!(LRound);
Accept!(LSquare);
Accept!(MinusAcc);
Accept!(MultiLineCommentAcc);
Accept!(OctalAcc);
Accept!(OrAcc);
Accept!(PercentAcc);
Accept!(PlusAcc);
Accept!(QuestionMark);
Accept!(RCurly);
Accept!(RRound);
Accept!(RSquare);
Accept!(Semicolon);
Accept!(SingleLineCommentAcc);
Accept!(SingleString);
Accept!(StarAcc);
Accept!(Tilde);
Accept!(WhiteSpace);
Accept!(Template);

State!(And);
State!(Assign);
State!(Bigger);
State!(Binary);
State!(Caret);
State!(Slash);
State!(Decimal);
State!(DecimalDigits);
State!(DecimalExponent);
State!(DecimalExponentSigned);
State!(Exclamation);
State!(Hex);
State!(InputElementDiv);
State!(Lesser);
State!(Minus);
State!(MultiLineComment);
State!(MultiLineCommentStar);
State!(Octal);
State!(Or);
State!(Percent);
State!(Plus);
State!(SawZero);
State!(SingleLineComment);
State!(Star);