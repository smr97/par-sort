import java.util.Arrays;

public class Bench {
    public static void main(String[] args) {
        int len = 100 * 1000 * 1000;
        long[] arr = new long[len];

        {
            for (int i = 0; i < len; ++i) {
                arr[i] = (long)i * i * i * 18913515181L;
            }

            long start = System.currentTimeMillis();
            Arrays.sort(arr);
            long end = System.currentTimeMillis();

            System.out.println("Arrays.sort          " + (end - start) + " ms");
        }

        {
            for (int i = 0; i < len; ++i) {
                arr[i] = (long)i * i * i * 18913515181L;
            }

            long start = System.currentTimeMillis();
            Arrays.parallelSort(arr);
            long end = System.currentTimeMillis();

            System.out.println("Arrays.parallelSort  " + (end - start) + " ms");
        }
    }
}
