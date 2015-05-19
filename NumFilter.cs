using System;
using System.Collections.Generic;
using System.Linq;

namespace NumFilter
{
    class Program
    {
        public static void Main()
        {
            var series = EmitSeries(new Random(), 20)
                .Take(10)
                .Where(n => (n & 1) == 0);

            foreach (var n in series)
            {
                Console.WriteLine(n);
            }
        }

        private static IEnumerable<int> EmitSeries(Random random, int max)
        {
            while (true)
            {
                yield return random.Next(max) + 1;
            }
        }
    }
}
