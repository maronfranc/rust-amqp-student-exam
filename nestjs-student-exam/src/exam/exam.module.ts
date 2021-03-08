import { Module } from '@nestjs/common';
import { ExamController } from './exam.controller';
import { ExamService } from './exam.service';
import { ExamQueueService } from './examQueue.service';

@Module({
  controllers: [ExamController],
  providers: [ExamService, ExamQueueService],
})
export class ExamModule { }
