use super::VariableDeclaration;

#[node]
pub enum DeclarationStatement {
    // Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}
