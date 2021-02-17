import { Body, Controller, Get, Param, ParseIntPipe, Post } from '@nestjs/common';
import { ExamDto } from './dto/Exam.dto';
import { ExamService } from './exam.service';

@Controller("exam")
export class ExamController {
  constructor(private readonly examService: ExamService) { }

  @Post("start")
  public async startExam(@Body() body: ExamDto): Promise<any> {
    const examData = body;

    void this.examService.emitStartExam(examData);

    return true;
  }

  @Post(":idExam/question/:idQuestion")
  /** emit to specific exam queue */
  public async answerQuestion(@Param("idExam", ParseIntPipe) idExam: number, @Param("idQuestion", ParseIntPipe) idQuestion: number): Promise<any> {
    const mockExamData: ExamDto = {
      id: idExam,
      content: "name",
    }

    void this.examService.sendQuestionAnswer(mockExamData);

    return true;
  }

  @Post(":idExam/finish")
  /** emit event to start consumption and close queue */
  public async finishExam(@Param("idExam", ParseIntPipe) idExam: number): Promise<any> {
    const mockExamData: ExamDto = {
      id: idExam,
      content: "name",
    }

    void this.examService.finishExam(mockExamData);

    return true;
  }

  @Get(":idExam/recover")
  /** attempt to get exam data in queue */
  public async recoverExam(@Param("idExam", ParseIntPipe) idExam: number): Promise<any> {

    return this.examService.getHello();
  }
}
