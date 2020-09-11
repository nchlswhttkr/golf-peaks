# Load data

load_run = function (filename) {
  run = read.csv(filename)
  run$process_time = run$user_time + run$system_time
  return(run)
}

data = list(
  "Initial run" = load_run("solver-benchmark-01.csv"),
  "Ignore paths with loops" = load_run("solver-benchmark-02.csv"),
  "Ignore paths with loops v2" = load_run("solver-benchmark-03.csv"),
  "Memoize repeated moves" = load_run("solver-benchmark-04.csv"),
  "Find shortest path" = load_run("solver-benchmark-05.csv"),
  "Disregard nonviable paths" = load_run("solver-benchmark-06.csv"),
  "Disregard nonviable paths v2" = load_run("solver-benchmark-07.csv")
)

# Average completion time between runs

measure_run_times = function(run) {
  time_per_iteration = aggregate(run$process_time, list(run$iteration), FUN=sum)$x
  mean_time_of_run = mean(time_per_iteration)
  variance = var(time_per_iteration)
  lower_bound = mean_time_of_run - 1.96 * sqrt(variance)
  upper_bound = mean_time_of_run + 1.96 * sqrt(variance)
  return(c(lower_bound, mean_time_of_run, upper_bound))
}

run_times = matrix(unlist(lapply(data, measure_run_times)), nrow=length(data), byrow=TRUE)

png(filename="./average-process-execution-time.png", width=800, height=450, res=96)
par(mar=c(7.5, 4, 4, 1))
plot(x=1:length(data), y=run_times[,2], type="n", ylim=c(0, 3), xlab="", xaxt="n", ylab="", frame.plot=FALSE)
title(main="Average process execution time (seconds)")
polygon(c(1:length(data), rev(1:length(data))), c(run_times[,1], rev(run_times[,3])), col="grey", border="grey")
lines(x=1:length(data), y=run_times[,2], col="blue")
abline(h=0:3, col="#CCCCCC99")
text(x=1:length(run_times), y=rep(-0.3, times=7), labels=names(data), adj=1, xpd=NA, srt=35)
dev.off()


# Level completion times across runs

get_average_level_time_normalised = function(run) {
  levels = aggregate(run$process_time, list(run$level), FUN=mean)$x
  return((levels - min(levels)) / (max(levels) - min(levels)))
}

level_times = lapply(data, get_average_level_time_normalised)

png(filename="./level-completion-time-normalised.png", width=800, height=450, res=96)
par(mar=c(7.5, 4, 4, 2))
plot(x=rep(1:length(level_times), each=120), y=unlist(level_times), xaxt="n", xlab="", yaxt="n", ylab="", frame.plot=FALSE)
title(main="Level completion time distribution, normalised across runs", cex.main=1.5)
abline(v=1:length(level_times), col="#CCCCCC99")
text(x=1:length(level_times), y=rep(-0.1, times=7), labels=names(level_times), adj=1, xpd=NA, srt=35)
dev.off()

# Name the costliest levels across each run

row_limit = 5;

get_longest_levels_of_run = function(run) {
  aggregated_times = aggregate(run$process_time, list(run$level), FUN=mean)
  sorted = order(-aggregated_times$x)
  aggregated_times$x[sorted]
  return(data.frame(
    level = aggregated_times$`Group.1`[sorted],
    times = aggregated_times$x[sorted]
  )[1:row_limit,])
}

longest_levels = lapply(data, get_longest_levels_of_run)

con = file("./longest-level-times.md", "wt")
for (name in  names(longest_levels)) {
  cat(paste("|", name), file=con)
}
cat("|\n", file=con)
cat(paste(rep("|-", length(longest_levels)), collapse = ""), file=con)
cat("|\n", file=con)
for (j in 1:row_limit) {
  for (i in 1:length(longest_levels)) {
    cat(sprintf("| `%s` (%.3fs) ", longest_levels[[i]][j, 'level'], longest_levels[[i]][j, 'times']), file=con)
  }
  cat("|\n", file=con)
}
close(con)
