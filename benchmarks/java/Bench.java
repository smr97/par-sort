import java.util.Arrays;
import java.util.Random;

public class Bench {
    public static void main(String[] args) {
        int len = 100 * 1000 * 1000;
        long[] arr = new long[len];

        {
            Random rnd = new Random();
            for (int i = 0; i < len; ++i) {
                arr[i] = rnd.nextLong();
            }

            long start = System.currentTimeMillis();
            Arrays.sort(arr);
            long end = System.currentTimeMillis();

            System.out.println("Arrays.sort          " + (end - start) + " ms");
        }

        {
            Random rnd = new Random();
            for (int i = 0; i < len; ++i) {
                arr[i] = rnd.nextLong();
            }

            long start = System.currentTimeMillis();
            Arrays.parallelSort(arr);
            long end = System.currentTimeMillis();

            System.out.println("Arrays.parallelSort  " + (end - start) + " ms");
        }
    }
}
