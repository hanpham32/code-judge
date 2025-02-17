pub const ASSERTIONS: [&str; 18] = [
    "Single Responsibility Principle (SRP): A class should only have a single responsibility, that is, only changes to one part of the software’s specification should be able to affect the specification of the class.",
    "Open/Closed Principle (OCP): “Software entities … should be open for extension, but closed for modification.”",
    "Liskov Substitution Principle (LSP): “Objects in a program should be replaceable with instances of their subtypes without altering the correctness of that program.”",
    "Interface Segregation Principle (ISP): “Many client-specific interfaces are better than one general-purpose interface.”",
    "Dependency Inversion Principle (DIP): One should “depend upon abstractions, [not] concretions.”",
    "Composite Reuse Principle (CRP): “a the principle that classes should favor polymorphic behavior and code reuse by their composition (by containing instances of other classes that implement the desired functionality) over inheritance from a base or parent class” - Knoernschild, Kirk (2002). Java Design - Objects, UML, and Process",
    "Don't Repeat Yourself (DRY): “a the principle that classes should favor polymorphic behavior and code reuse by their composition (by containing instances of other classes that implement the desired functionality) over inheritance from a base or parent class” - Knoernschild, Kirk (2002). Java Design - Objects, UML, and Process",
    "KISS: most systems work best if they are kept simple rather than made complicated; therefore, simplicity should be a key goal in design, and unnecessary complexity should be avoided",
    "Law of Demeter (LOD): a given object should assume as little as possible about the structure or properties of anything else (including its subcomponents), in accordance with the principle of “information hiding”",
    "Design by Contract (DbC): software designers should define formal, precise and verifiable interface specifications for software components, which extend the ordinary definition of abstract data types with preconditions, postconditions and invariants",
    "Encapsulation: bundling of data with the methods that operate on that data, or the restricting of direct access to some of an object’s components. Encapsulation is used to hide the values or state of a structured data object inside a class, preventing unauthorized parties’ direct access to them.",
    "Command-Query-Separation (CQS): “Functions should not produce abstract side effects…only commands (procedures) will be permitted to produce side effects.” - Bertrand Meyer: Object-Oriented Software Construction",
    "Principle of Least Astonishment (POLA): a component of a system should behave in a way that most users will expect it to behave. The behavior should not astonish or surprise users",
    "Linguistic-Modular-Unit: “Modules must correspond to syntactic units in the language used.” - Bertrand Meyer: Object-Oriented Software Construction",
    "Self-Documentation: Code should be readable and self-documenting, with meaningful names and clear structure, reducing the need for excessive comments.",
    "Uniform-Access: “All services offered by a module should be available through a uniform notation, which does not betray whether they are implemented through storage or through computation.” - Bertrand Meyer: Object-Oriented Software Construction",
    "Single-Choice: 
    “Whenever a software system must support a set of alternatives, one and only one module in the system should know their exhaustive list.” - Bertrand Meyer: Object-Oriented Software Construction",
    "Persistence-Closure: “Whenever a storage mechanism stores an object, it must store with it the dependents of that object. Whenever a retrieval mechanism retrieves a previously stored object, it must also retrieve any dependent of that object that has not yet been retrieved.” - Bertrand Meyer: Object-Oriented Software Construction",
];
