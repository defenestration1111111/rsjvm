public sealed class PermittedSubclasses permits Subclass, Subclass2 {
    public void function() { }
}

final class Subclass extends PermittedSubclasses {
}

final class Subclass2 extends PermittedSubclasses {
}
