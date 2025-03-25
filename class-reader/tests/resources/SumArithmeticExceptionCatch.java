public class SumArithmeticExceptionCatch {

    public static void main(String[] args) {
        int result = sum(10, 20);
    }

    public static int sum(int a, int b) {
        try {
            if (a == 0) {
                throw new ArithmeticException("Cannot divide by zero");
            }
            return a + b;
        } catch (ArithmeticException e) {
            return 0;
        }
    }
}
