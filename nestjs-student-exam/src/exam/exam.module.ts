import { Module } from '@nestjs/common';
import { ClientsModule } from '@nestjs/microservices';
import { ExamController } from './exam.controller';
import { ExamService } from './exam.service';
import { startExamProducer, ExamQueueService } from './examQueue.service';

@Module({
  imports: [ClientsModule.register([startExamProducer])],
  controllers: [ExamController],
  providers: [ExamService, ExamQueueService],
})
export class ExamModule { }
