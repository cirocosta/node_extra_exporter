# node_extra_exporter

![](https://hush-house.pivotal.io/api/v1/teams/main/pipelines/node_extra_exporter/badge)

> missing metrics from [`node_exporter`](https://github.com/prometheus/node_exporter)

## Metrics

### `schedstat`


```
node_schedstat_running_seconds_total{cpu="0"} 	123
node_schedstat_waiting_seconds_total{cpu="0"} 	123
node_schedstat_timeslices_total{cpu="0"} 	123
```


Sample output from `/proc/schedstat`:

```
      version 15
      timestamp 4300510924
   .--cpu0 0 0 0 0 0 0 383568852856 35528196627683 36508380
   |  domain0 7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   +--cpu1 0 0 0 0 0 0 379267286655 40702189769401 36446413
   |  domain0 7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   +--cpu2 0 0 0 0 0 0 398445452444 37677418991602 36546866
   |  domain0 7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   |
   *----> those we'll be parsing.
   
   
CPU STATS

    First field is a sched_yield() statistic:
         1) # of times sched_yield() was called

    Next three are schedule() statistics:
         2) This field is a legacy array expiration count field used in the O(1)
      scheduler. We kept it for ABI compatibility, but it is always set to zero.
         3) # of times schedule() was called
         4) # of times schedule() left the processor idle

    Next two are try_to_wake_up() statistics:
         5) # of times try_to_wake_up() was called
         6) # of times try_to_wake_up() was called to wake up the local cpu

  .--------------------------------------------------------------------------------.
  |                                                                                |
  | Next three are statistics describing scheduling latency:                       |
  |      7) sum of all time spent running by tasks on this processor (in jiffies)  |
  |      8) sum of all time spent waiting to run by tasks on this processor (in    |
  |         jiffies)                                                               |
  |      9) # of timeslices run on this cpu                                        |
  |                                                                                |
  *-----------------------THE ONES WE ACTUALLY CARE--------------------------------*

```
